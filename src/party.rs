use crate::utils::{aes::*, elligator::*,poly::*};
use rand::{CryptoRng, RngCore};

pub struct Party {
    pub set: Vec<[u8;32]>,
    pub party_type: bool, //Sender:True,Reciever:False
}

impl Party {
    pub fn new(list: Vec<[u8;32]>, party_type: bool) -> Party {
        Party {
            set: list,
            party_type: party_type,
        }
    }
    pub(crate) fn send_round1() -> [u8; 32] {
        let rng = rand::thread_rng();
        // 1. a ← KA.R
        let (privkeyshare, tweak) = key_from_rng(rng);
        // 2. m = KA.msg1(a)
        let msg = inverse_map(privkeyshare, tweak);
        msg
    }

    pub(crate) fn recv_round1(self) -> Vec<[u8; 32]> {
        //for i ∈ [n]:
        //  b_i ← KA.R
        //  m′_i =KA.msg2(b_i)
        //  f_i = Π^(−1) (m′i_)
        let mut privkeyshare_list: Vec<([u8; 32], u8)> = vec![];
        let mut f_i : Vec<[u8; 32]> = vec![];

        let n = self.set.len();
        for i in 0..n {
            let rng = rand::thread_rng();
            privkeyshare_list.push(key_from_rng(rng));
            let msg = inverse_map(privkeyshare_list[i].0, privkeyshare_list[i].1);
            f_i.push(permute(msg));
        }
        // P=interpol  ( {(H_1 (y_i),f_i)| y_i ∈Y})
        let x = to_scalar_vec(self.set);
        let y = to_scalar_vec(f_i);
        let poly = recover_pri_poly(x, y).unwrap();
        to_byte_array_vec(poly.coeffs)


    }

    pub(crate) fn send_round2() {}

    pub(crate) fn recv_round2() {}
}
