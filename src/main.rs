use aes_ctr::Aes256Ctr;
use aes_ctr::cipher::{
    generic_array::GenericArray,
    stream::{
        NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek
    }
};
use rand::Rng;
use std::fs;
use std::fs::*;
use std::io::prelude::*;
use std::io;

fn main() {
    
    // let mut testfile = fs::File::create(filename).unwrap();
    // testfile.write(b"fuck me daddy");
    
    let filename = "test.txt";
    
    //file encryption and decryption
    let key = generate_key();
    let nonce = encrypt_file(filename, &key).unwrap();
    println!("Nonce: {:?}", nonce);
    println!("Key: {:?}", key);

    decrypt_file(filename, &key, &nonce);
    
}

fn encrypt_file(filepath: &str, key: &[u8;32]) -> Result<[u8; 16], io::Error>{
    let mut filestream = fs::read(filepath)?;
    println!("before encryption: {:?}", filestream);

    let nonce = generate_nonce();
    let key = GenericArray::from_slice(key);
    
    let mut cipher = Aes256Ctr::new(key, &GenericArray::from_slice(&nonce));

    cipher.apply_keystream(&mut filestream);
    
    println!("after encryption: {:?}", filestream);

    let mut file = File::create(filepath)?;
    file.write(&filestream)?;
    

    Ok(nonce)
}

fn decrypt_file(filepath: &str, key: &[u8; 32], nonce: &[u8; 16]) -> Result<(), io::Error>{
    let mut filestream = fs::read(filepath)?;

    let key = GenericArray::from_slice(key);
    let nonce = GenericArray::from_slice(nonce);

    let mut cipher = Aes256Ctr::new(key, &nonce);

    cipher.seek(0);

    cipher.apply_keystream(&mut filestream);

    println!("DECRYPTED: {:?}", filestream);
    let mut file = File::create(filepath)?;
    file.write(&filestream)?;

    Ok(())
}

fn generate_nonce() -> [u8; 16]{
    let mut rng = rand::thread_rng();
    let mut nonce = [0u8; 16];

    for i in 0..16{
        nonce[i] = rng.gen_range(0..=255);
    }

    nonce
}

fn generate_key() -> [u8; 32]{
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 32];

    for i in 0..32{
        key[i] = rng.gen_range(0..=255);
    }

    key
}
