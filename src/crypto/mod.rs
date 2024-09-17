use std::io::{self, prelude::*, Cursor};

use aes::{
    cipher::{consts::U16, generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit},
    Aes128,
};
use flate2::{bufread::GzEncoder, Compression};
use flate2::{read::GzDecoder, write::ZlibEncoder};

use crate::config::{CHUNK_SIZE, KEY_BYTES};

fn pad_bytes(data: &mut Vec<u8>, chunk_size: usize) {
    let data_len = data.len();
    let target_len = if data_len % chunk_size != 0 {
        ((data_len / chunk_size) + 1) * chunk_size
    } else {
        data_len
    };

    if data_len < target_len {
        let pad_len = target_len - data_len;
        data.extend_from_slice(&vec![0; pad_len]);
    }
}

// assumes the input is already padded
pub fn split_vec_to_chunks(vec: &Vec<u8>, chunk_size: usize) -> Vec<Vec<u8>> {
    let iterations = vec.len() / chunk_size;
    let mut output: Vec<Vec<u8>> = Vec::new();

    for i in 0..iterations {
        let start = i * chunk_size;
        let end = (i + 1) * chunk_size;
        let chunk = vec[start..end].to_vec();

        output.push(chunk);
    }
    return output;
}

fn vec_to_array<const N: usize>(vec: Vec<u8>) -> Result<[u8; N], String> {
    if vec.len() == N {
        let boxed_slice = vec.into_boxed_slice();
        let boxed_array: Box<[u8; N]> = match boxed_slice.try_into() {
            Ok(arr) => arr,
            Err(_) => return Err("Conversion failed".into()),
        };
        Ok(*boxed_array)
    } else {
        Err(format!(
            "Vector has wrong length: expected {}, got {}",
            N,
            vec.len()
        ))
    }
}

fn encrypt_vec_block(cipher: &Aes128, vec_block: &Vec<u8>) -> Vec<u8> {
    let mut block_generic = GenericArray::clone_from_slice(vec_block);
    cipher.encrypt_block(&mut block_generic);

    block_generic.to_vec()
}

fn decrypt_vec_block(cipher: &Aes128, vec_block: &Vec<u8>) -> Result<Vec<u8>, String> {
    if vec_block.len() != 16 {
        return Err("Invalid block size".to_string());
    }

    let block_array: [u8; 16] = match vec_to_array::<16>(vec_block.clone()) {
        Ok(array) => array,
        Err(e) => return Err(e),
    };

    let mut block = GenericArray::<u8, U16>::clone_from_slice(&block_array);

    cipher.decrypt_block(&mut block);

    let decrypted_bytes = block.to_vec();

    Ok(decrypted_bytes)
}

pub fn encrypt_data(input: &Vec<u8>) -> Vec<u8> {
    let mut input_copy = input.clone();
    let original_size = input.len() as u32;
    pad_bytes(&mut input_copy, CHUNK_SIZE);

    let chunks = split_vec_to_chunks(&input_copy, CHUNK_SIZE);

    let key = GenericArray::from(KEY_BYTES);
    let cipher = Aes128::new(&key);

    let mut result: Vec<u8> = Vec::new();

    for chunk in chunks.iter() {
        let encrypted_bytes = encrypt_vec_block(&cipher, chunk);
        result.extend(&encrypted_bytes);
    }

    let size_buffer = original_size.to_be_bytes();

    let final_data: Vec<u8> = [&size_buffer[..], &result[..]].concat();

    return final_data;
}

pub fn compress_data(input: &Vec<u8>) -> io::Result<Vec<u8>> {
    let cursor = Cursor::new(input);
    let mut encoder = GzEncoder::new(cursor, Compression::default());
    let mut buffer = Vec::new();

    encoder.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn decompress_data(input: &Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
    let cursor = Cursor::new(input);
    let mut d = GzDecoder::new(cursor);
    let mut decompressed = Vec::new();

    d.read_to_end(&mut decompressed).unwrap();

    Ok(decompressed)
}

pub fn decrypt_data(input: &Vec<u8>) -> Result<Vec<u8>, String> {
    if input.len() < 4 {
        return Err("Failed".to_string());
    }

    let original_size_bytes: [u8; 4] = match input[0..4].try_into() {
        Ok(array) => array,
        Err(_) => return Err("Failed to read original size".to_string()),
    };

    let original_size = u32::from_be_bytes(original_size_bytes) as usize;

    let input_copy = if input.len() > 4 {
        input[4..].to_vec()
    } else {
        Vec::new()
    };

    let chunks = split_vec_to_chunks(&input_copy, CHUNK_SIZE);
    let key = GenericArray::from(KEY_BYTES);
    let cipher = Aes128::new(&key);

    let mut result = Vec::new();
    for chunk in chunks.iter() {
        match decrypt_vec_block(&cipher, chunk) {
            Ok(decrypted) => {
                result.extend(decrypted);
            }
            Err(_e) => eprintln!("Error decrypting chunk"),
        }
    }

    if original_size <= result.len() {
        result.truncate(original_size);
    } else {
        return Err("Error: Original size is larger than result length".to_string());
    }

    Ok(result)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_encrypt_data() {
        for _ in 0..100 {
            let data_len = rand::random::<usize>() % 1000;
            let padded_data_len = if data_len % CHUNK_SIZE != 0 {
                ((data_len / CHUNK_SIZE) + 1) * CHUNK_SIZE
            } else {
                data_len
            };

            let input: Vec<u8> = (0..data_len).map(|_| rand::random::<u8>()).collect();
            let encrypted = encrypt_data(&input);
            assert_eq!(padded_data_len + 4, encrypted.len());
        }
    }
    #[test]
    fn decrypt_data_test() {
        for _ in 0..100 {
            let data_len = rand::random::<usize>() % 1000;
            let input: Vec<u8> = (0..data_len).map(|_| rand::random::<u8>()).collect();
            let encrypted = encrypt_data(&input);
            let decrypted = decrypt_data(&encrypted).unwrap();

            assert_eq!(input, decrypted);
        }
    }
    #[test]
    fn encrypt_decrypt_large_data() {
        let data_len = 100000;
        let input: Vec<u8> = (0..data_len).map(|_| rand::random::<u8>()).collect();
        let encrypted = encrypt_data(&input);
        let decrypted = decrypt_data(&encrypted).unwrap();

        assert_eq!(input, decrypted);
    }
    #[test]
    fn test_compression() {
        for _ in 0..100 {
            let data_len = rand::random::<usize>() % 1000;
            let input: Vec<u8> = (0..data_len).map(|_| rand::random::<u8>()).collect();

            // let input = Vec::from("Hello world");
            let compressed = compress_data(&input);

            assert!(compressed.is_ok());
            let compressed = compressed.unwrap();
            let decompressed = decompress_data(&compressed);

            assert!(decompressed.is_ok());
            let decompressed = decompressed.unwrap();
            assert_eq!(input.len(), decompressed.len());
        }
    }
    #[test]
    fn test_compression_and_encryption() {
        let input = Vec::from("Hello world");
        let encrypted = encrypt_data(&input);
        let compressed = compress_data(&encrypted);

        assert!(compressed.is_ok());
        let compressed = compressed.unwrap();
        let decompressed = decompress_data(&compressed);
        assert!(decompressed.is_ok());

        let decrypted = decrypt_data(&decompressed.unwrap()).unwrap();
        assert_eq!(input, decrypted);
    }
    #[test]
    fn tests_compression_and_decryption() {
        for _ in 0..100 {
            let data_len = rand::random::<usize>() % 1000;
            let input: Vec<u8> = (0..data_len).map(|_| rand::random::<u8>()).collect();

            let encrypted = encrypt_data(&input);
            let compressed = compress_data(&encrypted);

            assert!(compressed.is_ok());
            let compressed = compressed.unwrap();
            let decompressed = decompress_data(&compressed);

            assert!(decompressed.is_ok());
            let decompressed = decompressed.unwrap();
            let decrypted = decrypt_data(&decompressed).unwrap();
            assert_eq!(input, decrypted);
        }
    }
    #[test]
    fn test_compression_and_encryption_reverse() {
        for _ in 0..100 {
            let data_len = rand::random::<usize>() % 1000;
            let input: Vec<u8> = (0..data_len).map(|_| rand::random::<u8>()).collect();

            let compressed = compress_data(&input);
            assert!(compressed.is_ok());
            let compressed = compressed.unwrap();
            let encrypted = encrypt_data(&compressed);
            let decrypted = decrypt_data(&encrypted).unwrap();
            let decompressed = decompress_data(&decrypted).unwrap();

            assert_eq!(input, decompressed);
        }
    }
}
