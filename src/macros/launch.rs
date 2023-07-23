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

// https://crates.io/crates/pkg-config
// https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
// https://doc.rust-lang.org/cargo/reference/workspaces.html
// https://doc.rust-lang.org/cargo/reference/build-script-examples.html