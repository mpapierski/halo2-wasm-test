#![no_main]

use std::io::Cursor;

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{bytesrepr::Bytes, ApiError};
use halo2_proofs::{
    plonk::{self, SingleVerifier},
    transcript::{Blake2bRead, Challenge255},
};
use shared::DefaultParams;

const VERIFIER_CONSTANT: u64 = 7;
const VERIFIER_A: u64 = 2;
const VERIFIER_B: u64 = 3;

#[no_mangle]
pub extern "C" fn call() {
    let proof_bytes: Bytes = runtime::get_named_arg("proof");
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(proof_bytes.as_slice());

    let params_bytes: Bytes = runtime::get_named_arg("params");
    let mut cur = Cursor::new(params_bytes.inner_bytes().clone());
    let params = DefaultParams::read(&mut cur)
        .map_err(|error| {
            runtime::print(&format!("Unable to read params {error}"));
            ApiError::User(50)
        })
        .unwrap_or_revert();
    let strategy = SingleVerifier::new(&params);

    let public_inputs = shared::make_public_inputs(VERIFIER_CONSTANT, VERIFIER_A, VERIFIER_B);
    let circuit = shared::make_circuit(VERIFIER_CONSTANT, VERIFIER_A, VERIFIER_B);

    let vk = plonk::keygen_vk(&params, &circuit)
        .map_err(|error| {
            runtime::print(&format!("keygen vk fail: {error}"));
            ApiError::User(51)
        })
        .unwrap_or_revert();

    match plonk::verify_proof(
        &params,
        &vk,
        strategy,
        &[&[public_inputs.as_slice()]],
        &mut transcript,
    ) {
        Ok(()) => {}
        Err(_error) => runtime::revert(ApiError::User(10)),
    };
}
