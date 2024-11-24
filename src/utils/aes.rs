use aes::cipher::InvalidLength;
use aes::Aes256;
use aes::cipher::{ generic_array::GenericArray,BlockEncrypt,BlockDecrypt, KeyInit};
use std::vec::Vec;

fn xor_block(block: &mut [u8; 16], round_key: &[u8; 16]) {
    for i in 0..16 {
        block[i] ^= round_key[i];
    }
}

fn aes_round_encrypt(block: &[u8; 16], round_key: &[u8; 16]) -> [u8; 16] {
   // let key = GenericArray::from_slice(round_key);  // wrap key in a GenericArray

    let cipher = Aes256::new_from_slice(round_key).expect("InvalidLength");
    let block_copy = *block;
    cipher.encrypt_block(&mut block_copy.into());
    block_copy
}

fn aes_round_decrypt(block: &[u8; 16], round_key: &[u8; 16]) -> [u8; 16] {
    //let key = GenericArray::from_slice(round_key);  // wrap key in a GenericArray

    let cipher = Aes256::new_from_slice(round_key).expect("InvalidLength");
    let block_copy = *block;
    cipher.decrypt_block(&mut block_copy.into());
    block_copy
}

fn ecb_enc_blocks(plaintexts: &Vec<[u8; 16]>, block_length: usize, round_keys: &Vec<[u8; 16]>) -> Vec<[u8; 16]> {
    let mut cyphertext = vec![[0u8; 16]; block_length];

    // Step for AES encryption
    let step = 8;
    //let mut idx = 0;
    let length = block_length - block_length % step;

    // Encrypt in chunks of 8 blocks
    for i in (0..length).step_by(step) {
        let mut temp = vec![[0u8; 16]; step];

        for j in 0..step {
            let mut block = plaintexts[i + j];
            xor_block(&mut block, &round_keys[0]);
            temp[j] = block;
        }

        // AES encryption rounds
        for round in 1..round_keys.len() - 1 {
            for j in 0..step {
                temp[j] = aes_round_encrypt(&temp[j], &round_keys[round]);
            }
        }

        // Final AES encryption with last round key
        for j in 0..step {
            temp[j] = aes_round_encrypt(&temp[j], &round_keys[round_keys.len() - 1]);
            cyphertext[i + j] = temp[j];
        }
    }

    // Encrypt remaining blocks
    for i in length..block_length {
        let mut block = plaintexts[i];
        xor_block(&mut block, &round_keys[0]);

        // Apply encryption rounds
        for round in 1..round_keys.len() - 1 {
            block = aes_round_encrypt(&block, &round_keys[round]);
        }

        // Final round encryption
        block = aes_round_encrypt(&block, &round_keys[round_keys.len() - 1]);
        cyphertext[i] = block;
    }

    cyphertext
}


fn ecb_dec_blocks(ciphertexts: &Vec<[u8; 16]>, block_length: usize, round_keys: &Vec<[u8; 16]>) -> Vec<[u8; 16]> {
    let mut plaintext = vec![[0u8; 16]; block_length];

    let step = 8;
    let length = block_length - block_length % step;

    // Decrypt in chunks of 8 blocks
    for i in (0..length).step_by(step) {
        let mut temp = vec![[0u8; 16]; step];

        for j in 0..step {
            let mut block = ciphertexts[i + j];
            // Apply decryption rounds
            block = aes_round_decrypt(&block, &round_keys[round_keys.len() - 1]);
            temp[j] = block;
        }

        // AES decryption rounds
        for round in (1..round_keys.len() - 1).rev() {
            for j in 0..step {
                temp[j] = aes_round_decrypt(&temp[j], &round_keys[round]);
            }
        }

        // XOR with the first round key to recover the original plaintext
        for j in 0..step {
            xor_block(&mut temp[j], &round_keys[0]);
            plaintext[i + j] = temp[j];
        }
    }

    // Decrypt remaining blocks
    for i in length..block_length {
        let mut block = ciphertexts[i];
        // Apply decryption rounds
        block = aes_round_decrypt(&block, &round_keys[round_keys.len() - 1]);
        
        // Decrypt remaining rounds
        for round in (1..round_keys.len() - 1).rev() {
            block = aes_round_decrypt(&block, &round_keys[round]);
        }

        // XOR with the first round key to recover the original plaintext
        xor_block(&mut block, &round_keys[0]);
        plaintext[i] = block;
    }

    plaintext
}



#[cfg(test)]
mod tests {
    use super::*;
    use aes::Aes256;
    use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};

    // Helper function to generate a sample round key
    fn generate_round_keys(key: &[u8; 32]) -> Vec<[u8; 16]> {
        let key = GenericArray::from_slice(key);
        let cipher = Aes256::new(key);
        let round_keys = vec![[0u8; 16]; 15];

        for i in 0..15 {
            cipher.encrypt_block(&mut round_keys[i].into());
        }

        round_keys
    }

    // Helper function to compare two vectors of blocks
    fn compare_blocks(blocks1: &Vec<[u8; 16]>, blocks2: &Vec<[u8; 16]>) -> bool {
        blocks1.iter().zip(blocks2.iter()).all(|(b1, b2)| b1 == b2)
    }

    // Test encryption and decryption with a simple case
    #[test]
    fn test_encryption_decryption() {
        let key: [u8; 32] = [0x00; 32];  // Simple key for testing
        let round_keys = generate_round_keys(&key);

        let plaintexts: Vec<[u8; 16]> = vec![
            [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10],
            [0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20]
        ];

        let ciphertexts = ecb_enc_blocks(&plaintexts, plaintexts.len(), &round_keys);
        let decrypted = ecb_dec_blocks(&ciphertexts, ciphertexts.len(), &round_keys);

        // Ensure the decrypted blocks match the original plaintext
        assert!(compare_blocks(&plaintexts, &decrypted), "Decrypted blocks do not match original plaintext");
    }

//     // Test encryption and decryption with an empty input
//     #[test]
//     fn test_empty_input() {
//         let key: [u8; 32] = [0x00; 32];
//         let round_keys = generate_round_keys(&key);

//         let plaintexts: Vec<[u8; 16]> = vec![];

//         let ciphertexts = ecb_enc_blocks(&plaintexts, plaintexts.len(), &round_keys);
//         let decrypted = ecb_dec_blocks(&ciphertexts, ciphertexts.len(), &round_keys);

//         // Ensure the decrypted blocks are also empty
//         assert_eq!(decrypted, plaintexts, "Decrypted blocks should be empty");
//     }

//     // Test case where plaintext is a single block
//     #[test]
//     fn test_single_block() {
//         let key: [u8; 32] = [0x00; 32];
//         let round_keys = generate_round_keys(&key);

//         let plaintexts: Vec<[u8; 16]> = vec![
//             [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10]
//         ];

//         let ciphertexts = ecb_enc_blocks(&plaintexts, plaintexts.len(), &round_keys);
//         let decrypted = ecb_dec_blocks(&ciphertexts, ciphertexts.len(), &round_keys);

//         // Ensure the decrypted block matches the original plaintext
//         assert_eq!(decrypted, plaintexts, "Decrypted block does not match original plaintext");
//     }

//     // Test case where plaintext is not a multiple of 8 blocks
//     #[test]
//     fn test_non_multiple_of_step() {
//         let key: [u8; 32] = [0x00; 32];
//         let round_keys = generate_round_keys(&key);

//         let plaintexts: Vec<[u8; 16]> = vec![
//             [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10],
//             [0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20],
//             [0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F, 0x30]
//         ];

//         let ciphertexts = ecb_enc_blocks(&plaintexts, plaintexts.len(), &round_keys);
//         let decrypted = ecb_dec_blocks(&ciphertexts, ciphertexts.len(), &round_keys);

//         // Ensure the decrypted blocks match the original plaintext
//         assert!(compare_blocks(&plaintexts, &decrypted), "Decrypted blocks do not match original plaintext");
//     }

//     // Test case with different keys for encryption and decryption
//     #[test]
//     fn test_different_keys() {
//         let key1: [u8; 32] = [0x00; 32];
//         let key2: [u8; 32] = [0x01; 32];
//         let round_keys1 = generate_round_keys(&key1);
//         let round_keys2 = generate_round_keys(&key2);

//         let plaintexts: Vec<[u8; 16]> = vec![
//             [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10]
//         ];

//         let ciphertexts = ecb_enc_blocks(&plaintexts, plaintexts.len(), &round_keys1);
//         let decrypted = ecb_dec_blocks(&ciphertexts, ciphertexts.len(), &round_keys2);

//         // Ensure the decryption with different keys results in an incorrect output
//         assert_ne!(plaintexts, decrypted, "Decryption with different keys should not match the original plaintext");
//     }
}
