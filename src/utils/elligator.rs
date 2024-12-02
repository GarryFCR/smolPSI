use curve25519_elligator2::{edwards::EdwardsPoint, MapToPointVariant, Randomized};
use rand::{CryptoRng, RngCore};

// pub struct Key {
//     pub privkey: [u8; 32],
//     pub pubkey: curve25519_elligator2::edwards::EdwardsPoint,
// }

// pub fn keygen() -> Key {
//     let mut rng = rand::thread_rng();
//     let mut privkey = [0_u8; 32];
//     rng.fill_bytes(&mut privkey);

//     let public_key = Randomized::mul_base_clamped(privkey);
//     Key {
//         privkey: privkey,
//         pubkey: public_key,
//     }
// }

pub fn key_from_rng<R: RngCore + CryptoRng>(mut csprng: R) -> ([u8; 32], u8) {
    let mut private = [0_u8; 32];
    csprng.fill_bytes(&mut private);

    // The tweak only needs generated once as it doesn't affect the overall
    // validity of the elligator2 representative.
    let tweak = csprng.next_u32() as u8;

    let mut repres: Option<[u8; 32]> = Randomized::to_representative(&private, tweak).into();
    let retry_limit: usize = 64;
    for _ in 0..retry_limit {
        if repres.is_some() {
            return (private, tweak);
        }
        csprng.fill_bytes(&mut private);
        repres = Randomized::to_representative(&private, tweak).into();
    }

    panic!("failed to generate representable secret, bad RNG provided");
}

pub fn inverse_map(privkeyshare: [u8; 32], tweak: u8) -> [u8; 32] {
    let rep: Option<[u8; 32]> = Randomized::to_representative(&privkeyshare, tweak).into();
    return rep.expect("Bad keyshare");
}

pub fn map(rep: [u8; 32]) -> EdwardsPoint {
    let  point: Option<EdwardsPoint> = Randomized::from_representative(&rep).into();
    return point.expect("Bad Representative");
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_map(){
        let rng = rand::thread_rng();
        let key = key_from_rng(rng);
        let rep = inverse_map(key.0,key.1);
        let point = map(rep);
        let point_byte1 = *point.compress().as_bytes();
        let point_byte2= *EdwardsPoint::mul_base_clamped(key.0).compress().as_bytes();
        assert_eq!(point_byte1,point_byte2);

    }
}