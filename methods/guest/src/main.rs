#![no_main]
// #![no_std]  // std support is experimental

use risc0_zkvm::guest::env;

use std::io::Read;
use fhe_traits::{DeserializeParametrized, Serialize};

use fhe::bfv::Ciphertext;
use fhe::bfv::BfvParametersBuilder;
risc0_zkvm::guest::entry!(main);

#[derive(serde::Serialize, serde::Deserialize)]
struct Input {
  ciph1: Vec<u8>,
  ciph2: Vec<u8>
}

pub fn main() {
  let start: usize = env::get_cycle_count();

  // Set same parameters as in host
  let parameters = BfvParametersBuilder::new()
      .set_degree(2048)
      .set_moduli(&[0x3fffffff000001])
      .set_plaintext_modulus(1 << 10)
      .build_arc().unwrap();

  // read the input from host
  let mut input_bytes = Vec::<u8>::new();
  env::stdin().read_to_end(&mut input_bytes).unwrap();
  let input: Input = bincode::deserialize(&input_bytes).unwrap();

  // Deserialize ciphertexts
  let ciph1: Ciphertext = Ciphertext::from_bytes(&input.ciph1, &parameters).unwrap();
  let ciph2: Ciphertext = Ciphertext::from_bytes(&input.ciph2, &parameters).unwrap();

  // Multiply ciphertexts
  let result = &ciph1 * &ciph2;

  // write public output to the journal
  env::commit(&result.to_bytes());
  let end = env::get_cycle_count();
  //total cycle count: 246.922.660 246934741 246932980
  eprintln!("total cycle count: {}", end - start);
}
