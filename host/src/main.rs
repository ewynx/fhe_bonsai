// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.

use methods::{
    METHOD_ELF, METHOD_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};

use fhe_traits::{FheDecoder, FheEncoder, FheEncrypter};
use fhe::bfv::{Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
use rand::{rngs::OsRng, thread_rng};
use fhe::bfv::BfvParametersBuilder;
use fhe_traits::{FheDecrypter, Serialize, DeserializeParametrized};

#[derive(serde::Serialize, serde::Deserialize)]
struct Input {
  ciph1: Vec<u8>,
  ciph2: Vec<u8>
}

fn main() {

  // Example from https://github.com/tlepoint/fhe.rs/blob/main/crates/fhe/README.md
  
    let parameters = BfvParametersBuilder::new()
      .set_degree(2048)
      .set_moduli(&[0x3fffffff000001])
      .set_plaintext_modulus(1 << 10)
      .build_arc().unwrap();
    let mut rng = thread_rng();

    let secret_key = SecretKey::random(&parameters, &mut OsRng);
    let public_key = PublicKey::new(&secret_key, &mut rng);

    let plaintext_1 = Plaintext::try_encode(&[20_u64], Encoding::poly(), &parameters).unwrap();
    let plaintext_2 = Plaintext::try_encode(&[-7_i64], Encoding::poly(), &parameters).unwrap();

    let ciphertext_1: Ciphertext = secret_key.try_encrypt(&plaintext_1, &mut rng).unwrap();
    let ciphertext_2: Ciphertext = public_key.try_encrypt(&plaintext_2, &mut rng).unwrap();

    let input = Input { ciph1: ciphertext_1.to_bytes(), ciph2: ciphertext_2.to_bytes() };

    let env = ExecutorEnv::builder()
      .write_slice(&bincode::serialize(&input).unwrap())
      .build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, METHOD_ELF).unwrap();

    // Deserialize resulting ciphertext
    let result: Vec<u8> = receipt.journal.decode().unwrap();
    let ciph_out: Ciphertext = Ciphertext::from_bytes(&result, &parameters).unwrap();

    receipt.verify(METHOD_ID).unwrap();

    let decrypted_plaintext = secret_key.try_decrypt(&ciph_out).unwrap();
    let decrypted_vector = Vec::<i64>::try_decode(&decrypted_plaintext, Encoding::poly()).unwrap();

    // Verify the result was correct
    assert_eq!(decrypted_vector[0], -140);
    println!("DONE");
}