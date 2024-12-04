use curve25519_elligator2::scalar::Scalar;
use sha2::{Digest, Sha256};

pub fn to_scalar(z: Vec<[u8; 32]>) -> Vec<Scalar> {
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

pub fn to_byte_array(scalars: Vec<Scalar>) -> Vec<[u8; 32]> {
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

#[cfg(test)]
mod tests {

    use super::*;

    // Test to check if `to_scalar` and `to_byte_array` are inverse functions
    #[test]
    fn test_to_scalar_and_to_byte_array_inverse() {
        // Step 1: Define a vector of 32-byte arrays (input)
        let input: Vec<[u8; 32]> = vec![
            [0x01; 32],
            [0x02; 32],
            [0x03; 32],
            [0x04; 32],
        ];

        // Step 2: Convert the byte arrays to Scalars using `to_scalar`
        let scalars = to_scalar(input.clone());

        // Step 3: Convert the Scalars back to byte arrays using `to_byte_array`
        let byte_arrays = to_byte_array(scalars);

        // Step 4: Verify that the byte arrays match the original input
        assert_eq!(input, byte_arrays, "The byte arrays should be the same after converting to scalars and back.");
    }

    #[test]
    fn test_to_scalar() {
        // Example input with two 32-byte arrays
        let input: Vec<[u8; 32]> = vec![
            [0u8; 32], 
            [1u8; 32]
        ];

        // Convert to scalars
        let result = to_scalar(input);

        // Check if the length of the result is as expected
        assert_eq!(result.len(), 2);

        let input1 = result[0].to_bytes();
        let input2 = result[1].to_bytes();

        // Check if the scalars are correct (for zero input, we expect the scalar to be 0)
        assert_eq!(input1, [0u8; 32]);
        assert_eq!(input2,  [1u8; 32]);  // As we used an array of 1's
    }
}
