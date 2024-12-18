use crate::utils::{aes::*, elligator::*, helper::*, poly::*};
use curve25519_elligator2::Scalar;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub enum Partytype {
    Sender,
    Receiver,
}
pub struct Party {
    pub set: Vec<String>,
    pub party_type: Partytype, //"Sender" or "Reciever"
    pub privkeyshare: Vec<[u8; 32]>,
}

impl Party {
    pub fn new(list: Vec<String>, party_type: Partytype) -> Party {
        Party {
            set: list,
            party_type: party_type,
            privkeyshare: vec![[0_u8; 32]],
        }
    }
    pub(crate) fn send_round1(&mut self) -> [u8; 32] {
        let rng = rand::thread_rng();
        // 1. a ← KA.R
        let (privkeyshare, tweak) = key_from_rng(rng);
        self.privkeyshare = vec![privkeyshare];
        // 2. m = KA.msg1(a)
        let msg = inverse_map(privkeyshare, tweak);
        msg
    }

    pub(crate) fn recv_round1(&mut self) -> (Vec<[u8; 32]>, Vec<[u8; 32]>) {
        //for i ∈ [n]:
        //  b_i ← KA.R
        //  m′_i =KA.msg2(b_i)
        //  f_i = Π^(−1) (m′i_)
        let mut privkeyshare_list: Vec<([u8; 32], u8)> = vec![];
        let mut f_i: Vec<[u8; 32]> = vec![];
        let mut extra: Vec<Scalar> = vec![];

        let n = self.set.len();
        for i in 0..n {
            let rng = rand::thread_rng();
            privkeyshare_list.push(key_from_rng(rng));
            let msg = inverse_map(
                privkeyshare_list[i].0.clone(),
                privkeyshare_list[i].1.clone(),
            );
            let mut inv = inverse_permute(msg);
            //this is done so that the most significant byte is not lost while performing 
            //operations due to reduction
            //todo: Find a better way to deal with this.
            extra.push(Scalar::from(inv[31]));
            inv[31] = 0;
            f_i.push(inv);
        }
        let b_i: Vec<[u8; 32]> = privkeyshare_list.into_iter().map(|(arr, _)| arr).collect();
        self.privkeyshare = b_i;
        // P=interpol  ( {(H_1 (y_i),f_i)| y_i ∈Y})
        let x = string_to_scalar(self.set.clone());
        let y = to_scalar(f_i);
        let poly1 = recover_pri_poly(x, y.clone()).unwrap();
        let poly2 = recover_pri_poly(y, extra).unwrap();

        (to_byte_array(poly1.coeffs), to_byte_array(poly2.coeffs))
    }

    pub(crate) fn send_round2(&mut self, coeff: (Vec<[u8; 32]>, Vec<[u8; 32]>)) -> Vec<[u8; 32]> {
        // (abort if deg(P ) < 1)
        if coeff.0.len() == 0 & coeff.1.len() {
            panic!("Polynomial of degree less than 1");
        }

        let poly1 = Poly {
            coeffs: to_scalar(coeff.0),
        };
        let poly2 = Poly {
            coeffs: to_scalar(coeff.1),
        };
        // 5. for i ∈ [n]:
        //      k_i = KA.key1(a,Π(P(H_1(x_i))))
        //      k_i′ = H_2 ( x_i , k_i )

        //H_1(x_i)
        let x = string_to_scalar(self.set.clone());
        let n = self.set.len();
        let mut k: Vec<[u8; 32]> = vec![[0; 32]; n];
        for i in 0..n {
            //P(H_1(x_i))
            let f_x = poly1.evaluate(x[i]);

            //Π(P(H_1(x_i)))
            let mut f_i = f_x.to_bytes();
            let f_xx = poly2.evaluate(f_x);
            f_i[31] = f_xx.to_bytes()[0];
            let permute = permute(f_i);

            //k_i = KA.key1(a,Π(P(H_1(x_i))))
            let edw_point = map(permute);
            if self.privkeyshare.len() != 1 {
                panic!("Incorrect length");
            }
            let a = self.privkeyshare[0];
            let k_i = edw_point.mul_clamped(a);
           // println!("{i} {:?}", k_i.compress().to_bytes().clone());

            //  k_i′ = H_2 ( x_i , k_i )
            k[i] = hash(vec![
                self.set[i].clone().as_bytes(),
                &k_i.compress().to_bytes(),
            ]);
        }
        //println!("\n\n\n");

        // 6. K = {k1′ ,...,kn′ } (shuffled)
        let mut rng = rand::thread_rng();
        k.shuffle(&mut rng);
        k
    }

    pub(crate) fn recv_round2(&mut self, k: Vec<[u8; 32]>, m: [u8; 32]) -> Vec<String> {
        let mut intersected_set = Vec::new();
        // output { y_i  | H_2 (y_i ,KA.key_2(b_i,m)) ∈ K }
        let n = self.set.len();
        let mut h: Vec<[u8; 32]> = vec![[0; 32]; n];

        for i in 0..n {
            //KA.key_2(b_i,m)
            let edw_point = map(m);
            let point = edw_point.mul_clamped(self.privkeyshare[i]);
            //println!("{i} {:?}", point.compress().to_bytes().clone());

            h[i] = hash(vec![
                self.set[i].clone().as_bytes(),
                &point.compress().to_bytes(),
            ]);
        }
        for i in 0..n {
            if k.contains(&h[i]) {
                intersected_set.push(self.set[i].clone());
            }
        }
        intersected_set
    }
}
