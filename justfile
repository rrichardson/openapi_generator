gen specs="openapi.yaml":
    RUST_LOG=info cargo run -- templates/rust {{specs}} -d output

watch directory="output":
    cd {{directory}} && cargo watch -x fmt -x "check --all-features --all-targets"
