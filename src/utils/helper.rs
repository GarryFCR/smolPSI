use curve25519_elligator2::scalar::Scalar;
use sha2::{Digest, Sha256};

pub fn byte_to_scalar(z: Vec<[u8; 32]>) -> Vec<Scalar> {
    let mut x: Vec<Scalar> = vec![];
    for i in z.iter() {
        x.push(Scalar::from_bytes_mod_order(*i));
    }
    x
}

pub fn string_to_scalar(z: Vec<String>) -> Vec<Scalar> {
    z.into_iter()
        .map(|s| {
            // Step 1: Convert the string into a byte vector
            let bytes = s.as_bytes();

            // Step 2: Hash the byte vector to ensure it's 32 bytes long
            let mut hasher = Sha256::new();
            hasher.update(bytes);
            let hash = hasher.finalize();

            // Step 3: Convert the first 32 bytes of the hash into a Scalar
            Scalar::from_bytes_mod_order(hash.into()) // Ensures the bytes fit within the scalar's field
        })
        .collect()
}

pub fn to_byte_array_vec(scalars: Vec<Scalar>) -> Vec<[u8; 32]> {
    let mut result: Vec<[u8; 32]> = vec![];

    for scalar in scalars.iter() {
        let bytes = scalar.to_bytes();
        result.push(bytes);
    }

    result
}

pub fn hash(data: Vec<&[u8]>)->[u8;32]{
    let mut hasher = Sha256::new();
    hasher.update(data.concat());
    let hash = hasher.finalize();
    hash.into()

}