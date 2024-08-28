use aes::{
    cipher::{consts::U16, generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit},
    Aes128,
};

use crate::config::{CHUNK_SIZE, KEY_BYTES};

fn pad_string(string: &str, chunk_size: usize) -> String {
    let target_len = if string.len() % chunk_size != 0 {
        ((string.len() / chunk_size) + 1) * chunk_size
    } else {
        string.len()
    };
    let mut padded = string.to_string();

    if string.len() < target_len {
        let pad_len = target_len - string.len();
        padded.push_str(&" ".repeat(pad_len));
    }

    return padded;
}

pub fn split_string_to_chunks(input: &str, chunk_size: usize) -> Vec<String> {
    let padded = pad_string(input, chunk_size);
    let iterations = padded.len() / chunk_size;

    let mut output: Vec<String> = Vec::with_capacity(iterations);

    for i in 0..iterations {
        let start = i * chunk_size;
        let end = (i + 1) * chunk_size;

        let chunk = &padded[start..end];
        println!("Pushing {}", i);
        output.push(chunk.to_string());
    }

    return output;
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

fn decrypt_vec_block(cipher: &Aes128, vec_block: &Vec<u8>) -> Result<String, String> {
    let block_array = match vec_to_array::<16>(vec_block.clone()) {
        Ok(array) => array,
        Err(e) => return Err(e),
    };

    let mut block = GenericArray::<u8, U16>::clone_from_slice(&block_array);
    cipher.decrypt_block(&mut block);

    let block_vec = block.to_vec();
    Ok(String::from_utf8_lossy(&block_vec).to_string())
}

pub fn encrypt_data(input: &Vec<u8>) -> Vec<u8> {
    let chunks = split_vec_to_chunks(input, CHUNK_SIZE);

    let key = GenericArray::from(KEY_BYTES);
    let cipher = Aes128::new(&key);

    let mut result: Vec<u8> = Vec::new();

    for chunk in chunks.iter() {
        let encrypted_string = encrypt_vec_block(&cipher, chunk);
        result.extend(&encrypted_string);
    }
    return result;
}

pub fn decrypt_data(input: &Vec<u8>) -> String {
    let chunks = split_vec_to_chunks(input, CHUNK_SIZE);
    let key = GenericArray::from(KEY_BYTES);
    let mut result = String::new();
    let cipher = Aes128::new(&key);
    for chunk in chunks.iter() {
        let decrypted_chunk = decrypt_vec_block(&cipher, chunk);
        match decrypted_chunk {
            Ok(decrypted) => result.push_str(&decrypted),
            Err(_e) => eprintln!("Error"),
        }
    }
    result.to_string()
}
