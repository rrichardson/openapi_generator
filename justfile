gen:
    RUST_LOG=info cargo run

watch directory="generated_server":
    cd {{directory}} && cargo +nightly watch -x fmt -x "check --all-features --all-targets"