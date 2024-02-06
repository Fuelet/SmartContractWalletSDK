use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use lib_flutter_rust_bridge_codegen::{
    config_parse, frb_codegen, get_symbols_if_no_duplicates, RawOpts,
};
use serde_json::Value;

const RUST_INPUT: &str = "src/api.rs";
const DART_OUTPUT: &str = "../lib/src/bridge_generated.dart";

const IOS_C_OUTPUT: &str = "../../flutter_fuelet_smart_contract_wallet/ios/Classes/frb.h";
const MACOS_C_OUTPUT_DIR: &str = "../../flutter_fuelet_smart_contract_wallet/macos/Classes/";

fn convert_bin_file_to_hex(path: String) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(hex::encode(&buffer))
}

const FORC_BINARIES: [(&str, &str); 3] = [
    ("../../../sway/secure_enclave_predicate/out/debug/secure_enclave_predicate.bin", "SECURE_ENCLAVE_PREDICATE_CODE"),
    ("../../../sway/recovery_checker/out/debug/recovery_checker.bin", "RECOVERY_CHECKER_CONTRACT_CODE"),
    ("../../../sway/withdrawal_script/out/debug/withdrawal_script.bin", "WITHDRAWAL_SCRIPT_CODE"),
];

const STORAGE_SLOTS_FILES: [(&str, &str); 1] = [
    ("../../../sway/recovery_checker/out/debug/recovery_checker-storage_slots.json", "RECOVERY_CHECKER_STORAGE_SLOTS"),
];

const GENERATED_CONSTS_PATH: &str = "src/gen_consts.rs";

/// Reads the binary and json files produced by Forc and stores them
/// as constants in the library sources
fn generate_forc_code_consts() -> io::Result<()> {
    let mut gen_content = "".to_string();

    for (bin_path, const_name) in FORC_BINARIES {
        let hex_code = convert_bin_file_to_hex(bin_path.to_string())?;
        gen_content = format!("{}pub const {}: &str = \"{}\";\n", gen_content, const_name, hex_code);
    }

    for (json_path, const_name) in STORAGE_SLOTS_FILES {
        let storage_json_string = fs::read_to_string(json_path)?;
        let slots: Vec<Value> = serde_json::from_str(&storage_json_string).unwrap();
        // let decoded_slots = serde_json::from_str::<Vec<StorageSlot>>(&storage_json_string)?;
        gen_content = format!("{}pub const {}: [(&str, &str); {}] = [\n", gen_content, const_name, slots.len());
        for slot in slots {
            gen_content = format!("{}    ({}, {}),\n", gen_content, slot.get("key").unwrap(), slot.get("value").unwrap());
        }
        gen_content = format!("{}];\n", gen_content);
    }

    let mut gen_consts_file = OpenOptions::new()
        .write(true)
        .create(true) // Create the file if it doesn't exist
        .truncate(true) // Truncate the file if it exists
        .open(GENERATED_CONSTS_PATH)?;

    gen_consts_file.write_all(gen_content.as_bytes())?;

    println!("Generated Forc constants");

    Ok(())
}

fn main() {
    // Tell Cargo that if the input Rust code changes, rerun this build script
    // println!("cargo:rerun-if-changed={}", RUST_INPUT);

    generate_forc_code_consts().unwrap();

    // Options for frb_codegen
    let raw_opts = RawOpts {
        rust_input: vec![RUST_INPUT.to_string()],
        dart_output: vec![DART_OUTPUT.to_string()],
        c_output: Some(vec![IOS_C_OUTPUT.to_string()]),
        extra_c_output_path: Some(vec![MACOS_C_OUTPUT_DIR.to_string()]),
        inline_rust: true,
        wasm: true,
        llvm_path: Some(vec!["/opt/homebrew/opt/llvm".to_string()]),
        ..Default::default()
    };

    // Generate Rust & Dart ffi bridges
    let configs = config_parse(raw_opts);
    let all_symbols = get_symbols_if_no_duplicates(&configs).unwrap();
    for config in configs.iter() {
        frb_codegen(config, &all_symbols).unwrap();
    }

    // Format the generated Dart code
    _ = std::process::Command::new("flutter")
        .arg("format")
        .arg("..")
        .spawn();
}
