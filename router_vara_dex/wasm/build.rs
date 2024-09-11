use router_vara_dex::RouterVaraDexProgram;
use std::{env, path::PathBuf};
use sails_client_gen::ClientGenerator;

fn main() {
    gwasm_builder::build();

    let idl_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("router_vara_dex.idl");

    let cargo_toml_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    sails_idl_gen::generate_idl_to_file::<RouterVaraDexProgram>(
        &idl_path,
    )
    .unwrap();

    ClientGenerator::from_idl_path(&idl_path)
    .with_mocks("with_mocks")
    .generate_to(cargo_toml_path.join("router_vara_dex_client.rs"))
    .unwrap();
}