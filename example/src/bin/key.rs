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

    //println!("{:?}", private_key.to_bytes());
    //println!("{:?}", public_key.to_bytes());

    let mut priv_file = File::create("private.txt").unwrap();
    priv_file.write_all(private_key.to_bytes().as_slice());
    let mut pub_file = File::create("public.txt").unwrap();
    pub_file.write_all(public_key.to_bytes().as_slice());
}
