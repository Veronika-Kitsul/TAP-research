extern crate openssl;

use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;

fn server_init() -> (<Kem as KemTrait>::PrivateKey, <Kem as KemTrait>::PublicKey) {
    // is this good? or the other one is better? should I write this into a file instead?
    let mut csprng = StdRng::from_entropy();

    // let mut file = File::create("private.txt");
    // file.write_all(Kem::pubkey);
    // let mut file = File::create("public.txt");
    // file.write_all(public_key);
    Kem::gen_keypair(&mut csprng)
}

fn main() {
    let passphrase = "generating-keys";
    let private_key, public_key = server_init();

    println!("Private key: {}", String::from_utf8(private_key).unwrap());
    println!("Public key: {}", String::from_utf8(public_key).unwrap());

    let mut file = File::create("private.txt")?;
    file.write_all(private_key)?;
    let mut file = File::create("public.txt")?;
    file.write_all(public_key)?;
}