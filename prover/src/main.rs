use std::{fs, io};

use halo2_proofs::{
    arithmetic::CurveAffine,
    plonk::{self, create_proof},
    poly::commitment::Params,
    transcript::{Blake2bWrite, Challenge255},
};
use rand::rngs::OsRng;

const PROVER_CONSTANT: u64 = 7;
const PROVER_A: u64 = 2;
const PROVER_B: u64 = 3;

fn params_to_bytes<C: CurveAffine>(params: Params<C>) -> io::Result<Vec<u8>> {
    let mut vec = Vec::new();
    params.write(&mut vec)?;
    Ok(vec)
}

fn main() {
    let params = shared::prepare_param();

    let circuit = shared::make_circuit(7, 2, 3);

    let vk = plonk::keygen_vk(&params, &circuit).unwrap();
    let pk = plonk::keygen_pk(&params, vk, &circuit).unwrap();

    let public_inputs = shared::make_public_inputs(PROVER_CONSTANT, PROVER_A, PROVER_B);

    let rng = OsRng;

    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);

    println!("creating proof...");

    create_proof(
        &params,
        &pk,
        &[circuit],
        &[&[public_inputs.as_slice()]],
        rng,
        &mut transcript,
    )
    .expect("should create proof");

    let params_bytes = params_to_bytes(params).unwrap();

    let proof = transcript.finalize();

    println!("Proof size: {}", proof.len());
    println!("Params size: {}", params_bytes.len());

    fs::write("/tmp/proof.bin", &proof).unwrap();
    fs::write("/tmp/params.bin", &params_bytes).unwrap();
}
