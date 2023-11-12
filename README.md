# rust-workspace ![](https://github.com/nirmangupta/rust-workspace/workflows/Rust/badge.svg)
This repository contains template code for a new rust project.

### Usage
* To use this repository, clone it as a template.  
* Add a new binary or library either using `cargo new` or by manually creating a folder.  
* Add the name of the package to workspace's `Cargo.toml` under `workspace.members`.  
More details about cargo workspace at https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

### Notes
* Important to replace the `LICENSE` as per project requirement.
* Edit the `README` to show case project usage and features.
* Basic CI pipeline using github actions can be found under `/github/rust.yml`.