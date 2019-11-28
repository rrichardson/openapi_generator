gen specs="openapi.yaml":
    RUST_LOG=info cargo run {{specs}}

watch directory="output":
    cd {{directory}} && cargo +nightly watch -x fmt -x check
