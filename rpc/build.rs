use std::{path::Path, process::Command};

extern crate tonic_build;

fn main() {
    if !Path::new("api/.git").exists() {
        let output = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .output()
            .expect("failed to execute git command ");

        if !output.status.success() {
            panic!("submodule checkout failed");
        }
    }

    tonic_build::configure()
        .build_server(false)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["api/protobuf/mayastor.proto"], &["api/protobuf"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
