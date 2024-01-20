use std::{path::PathBuf, sync::Arc};

use foundry_compilers::{
    artifacts::{output_selection::OutputSelection, Settings, Source, Sources},
    remappings::Remapping,
    CompilerInput, Solc,
};

fn main() {
    let mut settings = Settings::new(OutputSelection::default());
    settings.evm_version = Some(foundry_compilers::EvmVersion::Byzantium);
    settings.remappings.push(Remapping {
        context: None,
        name: "project".to_string(), // if set to `project/`, the compilation will success.
        path: "src".to_string(),
    });
    println!("{}", serde_json::to_string(&settings).unwrap());

    let mut sources = Sources::new();
    sources.insert(
        PathBuf::from("src/contract.sol"),
        Source {
            content: Arc::new(include_str!("./contract.sol").to_string()),
        },
    );
    sources.insert(
        PathBuf::from("src/dependency.sol"),
        Source {
            content: Arc::new(include_str!("./dependency.sol").to_string()),
        },
    );

    let input = CompilerInput {
        language: "Solidity".to_string(),
        sources,
        settings,
    };
    println!("{}", serde_json::to_string(&input).unwrap());

    let compiler = Solc::find_or_install_svm_version("0.6.8").unwrap();
    let output = compiler.compile_exact(&input).unwrap();
    println!("{}", serde_json::to_string(&output).unwrap());

    // Compilation should be success if the remapping is `project=src`.
    // However, foundry-rs/compilers output the remapping as `project=src/`, which leads to file not found error.
    assert!(!output.has_error());
}
