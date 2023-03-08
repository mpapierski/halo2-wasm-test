use std::fs;

use casper_engine_test_support::{InMemoryWasmTestBuilder, PRODUCTION_RUN_GENESIS_REQUEST, ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR, DeployItemBuilder};
use casper_types::{bytesrepr::Bytes, runtime_args, RuntimeArgs, system::standard_payment, U512};

#[test]
fn should_execute() {
    let proof = fs::read("/tmp/proof.bin").unwrap();
    let params = fs::read("/tmp/params.bin").unwrap();

    let mut builder = InMemoryWasmTestBuilder::default();

    builder.run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST);
    let exec_request = {
        let account_hash = *DEFAULT_ACCOUNT_ADDR;
        let session_args = runtime_args! {
            "proof" => Bytes::from(proof),
            "params" => Bytes::from(params),
        };

        let deploy_hash: [u8; 32] = [123; 32];

        let payment_amount = U512::from(10_000_000_000_000_000u64);

        let deploy = DeployItemBuilder::new()
            .with_address(account_hash)
            .with_session_code("halo2_wasm_test.wasm", session_args)
            .with_empty_payment_bytes(runtime_args! {
                standard_payment::ARG_AMOUNT => payment_amount,
            })
            .with_authorization_keys(&[account_hash])
            .with_deploy_hash(deploy_hash)
            .build();

        ExecuteRequestBuilder::new().push_deploy(deploy)
    }
    .build();
    builder.exec(exec_request).expect_success().commit();

    println!("gas {}", builder.last_exec_gas_cost());
}
