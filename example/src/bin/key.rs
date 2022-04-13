use std::io::Read;
use std::io::BufReader;
use hpke::{
    kem::X25519HkdfSha256,
    Kem as KemTrait,
    Serializable,
};
use rand::{rngs::StdRng, SeedableRng};
use std::{fs::File, io::Write};


type Kem = X25519HkdfSha256;

fn server_init() -> (<Kem as KemTrait>::PrivateKey, <Kem as KemTrait>::PublicKey) {
    let mut csprng = StdRng::from_entropy();
    Kem::gen_keypair(&mut csprng)
}

fn main() {
    let (private_key, public_key) = server_init();

    let mut priv_file = File::create("private.txt").unwrap();
    priv_file.write_all(private_key.to_bytes().as_slice()).unwrap();
    let mut pub_file = File::create("public.txt").unwrap();
    pub_file.write_all(public_key.to_bytes().as_slice()).unwrap();

    // let f = File::open("private.txt").unwrap();
    // let mut reader = BufReader::new(f);
    // let mut priv_key_bytes = Vec::new();
    
    // Read file into vector.
    // reader.read_to_end(&mut priv_key_bytes).unwrap();

    // let private_key2: <Kem as KemTrait>::PrivateKey =
        // Deserializable::from_bytes(&priv_key_bytes).unwrap();
}
