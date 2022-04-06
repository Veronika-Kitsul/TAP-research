use std::fs::File;
use std::io::Write;
use hpke::Kem


fn server_init() -> (<Kem as KemTrait>::PrivateKey, <Kem as KemTrait>::PublicKey) {
    // is this good? or the other one is better? should I write this into a file instead?
    let mut csprng = StdRng::from_entropy();
    Kem::gen_keypair(&mut csprng)
}

fn main() {
    let (private_key, public_key) = server_init();

    println!("Private key: {}", String::from_utf8(private_key).unwrap());
    println!("Public key: {}", String::from_utf8(public_key).unwrap());

    let mut file = File::create("private.txt");
    file.write_all(private_key);
    let mut file = File::create("public.txt");
    file.write_all(public_key);
}