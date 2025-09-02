use std::collections::HashSet;
use prost::Message;
use prost_types::FileDescriptorSet;
use std::fs;
use std::io::Result;
use heck::ToUpperCamelCase;

const PROTOS: &[&str] = &[
    // COSMOS
    "cosmos/app/v1alpha1/query.proto",
    "cosmos/auth/v1beta1/query.proto",
    "cosmos/auth/v1beta1/tx.proto",
    "cosmos/authz/v1beta1/query.proto",
    "cosmos/authz/v1beta1/tx.proto",
    "cosmos/autocli/v1/query.proto",
    "cosmos/bank/v1beta1/query.proto",
    "cosmos/bank/v1beta1/tx.proto",
    "cosmos/base/node/v1beta1/query.proto",
    "cosmos/base/tendermint/v1beta1/query.proto",
    "cosmos/circuit/v1/query.proto",
    "cosmos/circuit/v1/tx.proto",
    "cosmos/consensus/v1/query.proto",
    "cosmos/consensus/v1/tx.proto",
    "cosmos/crisis/v1beta1/tx.proto",
    "cosmos/distribution/v1beta1/query.proto",
    "cosmos/distribution/v1beta1/tx.proto",
    "cosmos/evidence/v1beta1/query.proto",
    "cosmos/evidence/v1beta1/tx.proto",
    "cosmos/feegrant/v1beta1/query.proto",
    "cosmos/feegrant/v1beta1/tx.proto",
    "cosmos/gov/v1/query.proto",
    "cosmos/gov/v1/tx.proto",
    "cosmos/gov/v1beta1/query.proto",
    "cosmos/gov/v1beta1/tx.proto",
    "cosmos/group/v1/query.proto",
    "cosmos/group/v1/tx.proto",
    "cosmos/mint/v1beta1/query.proto",
    "cosmos/mint/v1beta1/tx.proto",
    "cosmos/nft/v1beta1/query.proto",
    "cosmos/nft/v1beta1/tx.proto",
    "cosmos/orm/query/v1alpha1/query.proto",
    "cosmos/params/v1beta1/query.proto",
    "cosmos/query/v1/query.proto",
    "cosmos/slashing/v1beta1/query.proto",
    "cosmos/slashing/v1beta1/tx.proto",
    "cosmos/staking/v1beta1/query.proto",
    "cosmos/staking/v1beta1/tx.proto",
    "cosmos/tx/v1beta1/service.proto",
    "cosmos/tx/v1beta1/tx.proto",
    "cosmos/upgrade/v1beta1/query.proto",
    "cosmos/upgrade/v1beta1/tx.proto",
    "cosmos/vesting/v1beta1/tx.proto",
    // IBC
    "ibc/applications/transfer/v1/query.proto",
    "ibc/applications/transfer/v1/tx.proto",
    "ibc/core/channel/v1/query.proto",
    "ibc/core/channel/v1/tx.proto",
    "ibc/core/client/v1/query.proto",
    "ibc/core/client/v1/tx.proto",
    "ibc/core/connection/v1/query.proto",
    "ibc/core/connection/v1/tx.proto",
    "ibc/core/port/v1/query.proto",
    // POA
    "strangelove_ventures/poa/v1/query.proto",
    "strangelove_ventures/poa/v1/tx.proto",
    // TokenFactory
    "osmosis/tokenfactory/v1beta1/query.proto",
    "osmosis/tokenfactory/v1beta1/tx.proto",
    // Manifest
    "liftedinit/manifest/v1/query.proto",
    "liftedinit/manifest/v1/tx.proto",
];

fn main() -> Result<()> {
    let out_dir = "src/gen";
    std::fs::create_dir_all(out_dir)?;
    println!("cargo:rerun-if-changed=proto");

    let desc_path = format!("{}/descriptors.bin", out_dir);

    prost_build::Config::new()
        .include_file("mod.rs") // Generates a file which contains a set of pub mod XXX statements combining to load all Rust files generated.
        .out_dir(out_dir)
        .file_descriptor_set_path(&desc_path)
        .compile_well_known_types()
        .compile_protos(PROTOS,&["proto"])?;

    let bytes = fs::read(&desc_path)?;
    let fds = FileDescriptorSet::decode(bytes.as_slice())?;

    let allowed_files: HashSet<String> = PROTOS
        .iter()
        .map(|p| p.trim_start_matches("proto/").to_string())
        .collect();

    let mut allowed_pkgs: HashSet<String> = HashSet::new();
    for file in &fds.file {
        if let (Some(name), Some(pkg)) = (file.name.as_ref(), file.package.as_ref()) {
            if !pkg.is_empty() && allowed_files.contains(name) {
                allowed_pkgs.insert(pkg.clone());
            }
        }
    }

    let mut out = String::from("// @generated: type URLs for messages\n");
    out.push_str("// Do not edit; see build.rs\n\n");

    for file in &fds.file {
        let pkg = file.package.as_deref().unwrap_or("");
        if pkg.is_empty() {
            continue;
        }
        // Skip packages not explicitly listed, and well-known types.
        if !allowed_pkgs.contains(pkg) {
            continue;
        }

        let rust_pkg = pkg.replace('.', "::");
        for md in &file.message_type {
            let name = md.name.as_deref().unwrap_or("");
            if name.is_empty() {
                continue;
            }

            let full_type_url = format!("/{pkg}.{name}");
            let rust_ident = name.to_upper_camel_case();
            let rust_path = format!("crate::{}::{}", rust_pkg, rust_ident);
            out.push_str(&format!(
                "impl {rust_path} {{ pub const TYPE_URL: &'static str = \"{full_type_url}\"; }}\n"
            ));
        }
    }

    fs::write(format!("{}/type_urls.rs", out_dir), out)?;
    Ok(())
}
