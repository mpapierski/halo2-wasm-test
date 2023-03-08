halo2-wasm-test
===

- `contract` - verifier contract that accepts `proof` and `params`
- `prover` - prover program that generates `proof.bin` and `params.bin`
- `tests` - test that runs the contract under wasm.

# Running

- max_memory, and max_stack_height requires increase to properly run the contract under test environment.
