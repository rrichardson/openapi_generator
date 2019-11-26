gen:
    RUST_LOG=info cargo run

watch directory="output":
    cd {{directory}} && cargo +nightly watch -x fmt -x "check --all-features --all-targets"