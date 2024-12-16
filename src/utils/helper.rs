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

pub fn hash(data: Vec<&[u8]>) -> [u8; 32] {
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
            [
                187, 180, 56, 208, 215, 198, 166, 114, 114, 197, 120, 218, 30, 30, 124, 213, 238,
                136, 18, 31, 227, 68, 200, 96, 20, 157, 79, 28, 192, 86, 6, 15,
            ],
            [
                241, 223, 178, 205, 173, 213, 90, 61, 250, 2, 16, 12, 197, 69, 52, 150, 13, 237,
                218, 149, 56, 91, 182, 98, 254, 36, 179, 155, 89, 41, 168, 15,
            ],
            [
                173, 69, 140, 100, 94, 114, 135, 30, 150, 212, 171, 83, 89, 55, 82, 108, 143, 223,
                199, 103, 188, 182, 114, 201, 71, 206, 213, 183, 105, 85, 174, 15,
            ],
            [
                137, 219, 49, 179, 24, 10, 177, 24, 50, 100, 22, 111, 86, 197, 215, 27, 114, 110,
                228, 63, 207, 240, 52, 245, 195, 194, 111, 23, 242, 7, 145, 15,
            ],
        ];

        // Step 2: Convert the byte arrays to Scalars using `to_scalar`
        let scalars = to_scalar(input.clone());

        // Step 3: Convert the Scalars back to byte arrays using `to_byte_array`
        let byte_arrays = to_byte_array(scalars);

        // Step 4: Verify that the byte arrays match the original input
        assert_eq!(
            input, byte_arrays,
            "The byte arrays should be the same after converting to scalars and back."
        );
    }

    #[test]
    fn test_to_scalar() {
        // Example input with two 32-byte arrays
        let input: Vec<[u8; 32]> = vec![[0u8; 32], [1u8; 32]];

        // Convert to scalars
        let result = to_scalar(input);

        // Check if the length of the result is as expected
        assert_eq!(result.len(), 2);

        let input1 = result[0].to_bytes();
        let input2 = result[1].to_bytes();

        // Check if the scalars are correct (for zero input, we expect the scalar to be 0)
        assert_eq!(input1, [0u8; 32]);
        assert_eq!(input2, [1u8; 32]); // As we used an array of 1's
    }
}
