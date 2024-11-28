use curve25519_elligator2::{edwards::EdwardsPoint, MapToPointVariant, Randomized};
use rand::{CryptoRng, RngCore};
const RETRY_LIMIT: usize = 64;

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

    for _ in 0..RETRY_LIMIT {
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
