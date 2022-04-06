extern crate openssl;

use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;

fn main() {
    let passphrase = "generating-keys";

    let rsa = Rsa::generate(1024).unwrap();
    let private_key: Vec<u8> = rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), passphrase.as_bytes()).unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();

    println!("Private key: {}", String::from_utf8(private_key).unwrap());
    println!("Public key: {}", String::from_utf8(public_key).unwrap());
    // the problem is my key is the same as in the example and I don't know why

    let mut file = File::create("private.txt")?;
    file.write_all(private_key)?;
    let mut file = File::create("public.txt")?;
    file.write_all(public_key)?;
    // how do i nicely store both keys? separate files?
}