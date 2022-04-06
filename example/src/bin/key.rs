//extern crate openssl;
//use openssl::rsa::{Rsa, Padding};
//use openssl::symm::Cipher;
use hpke::{
    kem::X25519HkdfSha256,
    Kem as KemTrait,
    Serializable,
};
use rand::{rngs::StdRng, SeedableRng};
use std::{fs::File, io::Write};


type Kem = X25519HkdfSha256;

fn server_init() -> (<Kem as KemTrait>::PrivateKey, <Kem as KemTrait>::PublicKey) {
    // is this good? or the other one is better? should I write this into a file instead?
    let mut csprng = StdRng::from_entropy();
    Kem::gen_keypair(&mut csprng)
}

fn main() {
    let (private_key, public_key) = server_init();

    println!("{:?}", private_key.to_bytes());
    println!("{:?}", public_key.to_bytes());

    //println!("Private key: {}", String::from_utf8(private_key).unwrap());
    //println!("Public key: {}", String::from_utf8(public_key).unwrap());

    let mut file = File::create("private.txt");
    file.write(private_key.to_bytes());
    let mut file = File::create("public.txt");
    std::fs::write(public.txt, public_key.to_bytes());
}
