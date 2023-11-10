//! Build config

// Codegen config
const SERDE_DERIVE_MACRO: &str = "#[derive(serde::Deserialize, serde::Serialize)]";
const OUT_DIR: &str = "./src/net/rpc/";

// Protobuf config
const PROTO_DIR: &str = "src/net/rpc/proto/";
const PROTO_FILE: &str = "src/net/rpc/proto/api.proto";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // RPC config
    tonic_build::configure()
        .build_server(true)
        .out_dir(OUT_DIR)
        .type_attribute("insert", SERDE_DERIVE_MACRO)
        .type_attribute("metadata", SERDE_DERIVE_MACRO)
        .type_attribute("parameter", SERDE_DERIVE_MACRO)
        .compile(&[PROTO_FILE], &[PROTO_DIR])?;
    Ok(())
}
