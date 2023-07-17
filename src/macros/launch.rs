#[macro_export]
macro_rules! register_launch_group {
    ($binaries: expr) => {
        fn main() {
            // WIP
            use std::collections::HashMap;

            let binaries = HashMap::from($binaries);

            dbg!(binaries);
        }
    };
}
/*use std::fs;
use std::process::Command;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Members {
    members: Vec<String>
}

#[derive(Debug, Deserialize)]
struct Workspace {
    workspace: Members
}

fn main() {

    let content = match fs::read_to_string("Cargo.toml") {
        Ok(c) => c,
        Err(error) => {
            eprintln!("Error: {}", error.to_string());
            panic!("Could not open workspace");
        }
    };

    let workspace: Workspace = match toml::from_str(content.as_str()) {
        Ok(pkg) => pkg,
        Err(error) => {
            eprintln!("Error: {}", error.message());
            panic!("Could not parse workspace");
        }
    };

    for member in test.workspace.members {
        let path = member + "\\Cargo.toml";
        let status = Command::new("cargo")
            .args(&["run", "--manifest-path", path.as_str()])
            .status()
            .unwrap();

        dbg!(status);
    }

    println!("Hello, world!");
}
*/
// https://crates.io/crates/pkg-config
// https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
// https://doc.rust-lang.org/cargo/reference/workspaces.html
// https://doc.rust-lang.org/cargo/reference/build-script-examples.html