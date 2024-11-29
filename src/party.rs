use crate::utils::{aes::*, elligator::*, poly::*,helper::*};
use curve25519_elligator2::edwards;
use rand::{seq::SliceRandom, CryptoRng, RngCore};

pub struct Party {
    pub set: Vec<String>,
    pub party_type: bool, //Sender:True,Reciever:False
    pub privkeyshare : Vec<[u8; 32]>
}

impl Party {
    pub fn new(list: Vec<String>, party_type: bool) -> Party {
         Party {
            set: list,
            party_type: party_type,
            privkeyshare: vec![[0_u8;32]]
        }
        
    }
    pub(crate) fn send_round1(mut self) -> [u8; 32] {
        let rng = rand::thread_rng();
        // 1. a ← KA.R
        let (privkeyshare, tweak) = key_from_rng(rng);
        self.privkeyshare = vec![privkeyshare];
        // 2. m = KA.msg1(a)
        let msg = inverse_map(privkeyshare, tweak);
        msg
    }

    pub(crate) fn recv_round1(mut self) -> Vec<[u8; 32]> {
        //for i ∈ [n]:
        //  b_i ← KA.R
        //  m′_i =KA.msg2(b_i)
        //  f_i = Π^(−1) (m′i_)
        let mut privkeyshare_list: Vec<([u8; 32], u8)> = vec![];
        let mut f_i: Vec<[u8; 32]> = vec![];

        let n = self.set.len();
        for i in 0..n {
            let rng = rand::thread_rng();
            privkeyshare_list.push(key_from_rng(rng));
            let msg = inverse_map(privkeyshare_list[i].0, privkeyshare_list[i].1);
            f_i.push(inverse_permute(msg));
        }
        let b_i: Vec<[u8;32]> = privkeyshare_list.into_iter().map(|(arr, _)| arr).collect();
        self.privkeyshare = b_i;
        // P=interpol  ( {(H_1 (y_i),f_i)| y_i ∈Y})
        let x = string_to_scalar(self.set.clone());
        let y = byte_to_scalar(f_i);
        let poly = recover_pri_poly(x, y).unwrap();
        to_byte_array_vec(poly.coeffs)
    }

    pub(crate) fn send_round2(self, coeff: Vec<[u8; 32]>)->Vec<[u8;32]> {
        // (abort if deg(P ) < 1)
        if coeff.len() == 0 {
            panic!("Polynomial of degree less than 1");
        }

        let poly = Poly{
            coeffs:byte_to_scalar(coeff)
        };
        // 5. for i ∈ [n]:
        //      k_i = KA.key1(a,Π(P(H_1(x_i))))
        //      k_i′ = H_2 ( x_i , k_i )

        //H_1(x_i)
        let x = string_to_scalar(self.set.clone());
        let n = self.set.len();
        let mut K : Vec<[u8;32]> =vec![[0; 32]; n];
        for i in 0..n {
            //P(H_1(x_i))
            let f_x = poly.evaluate(x[i]);
            //Π(P(H_1(x_i)))
            let permute = permute(f_x.to_bytes());
            //k_i = KA.key1(a,Π(P(H_1(x_i))))
            let edw_point = map(permute);
            if self.privkeyshare.len() != 1 {
                panic!("Incorrect length");

            }
            let a = self.privkeyshare[0];
            let k_i = edw_point.mul_clamped(a);
            //  k_i′ = H_2 ( x_i , k_i )
            K[i] = hash(vec![self.set[i].clone().as_bytes(),&k_i.compress().to_bytes()]);

        }

        // 6. K = {k1′ ,...,kn′ } (shuffled)
        let mut rng = rand::thread_rng();
        K.shuffle(&mut rng);
        K
    }

    pub(crate) fn recv_round2(self, K :Vec<[u8;32]>,m :[u8; 32])->Vec<String> {
        let mut intersected_set = Vec::new(); 
        // output { y_i  | H_2 (y_i ,KA.key_2(b_i)) ∈ K }
        let n = self.set.len();
        let mut H : Vec<[u8;32]> =vec![[0; 32]; n];
        for i in 0..n{
            H[i] = hash(vec![self.set[i].clone().as_bytes(),&m.clone()]);
        }
        for i in 0..n{
            if K.contains(&H[i]){
                intersected_set.push(self.set[i].clone());
            }
        }
        intersected_set
    }
}

