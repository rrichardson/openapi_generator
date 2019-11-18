gen:
    RUST_LOG=info cargo run

watch:
    cd generated_server && cargo watch -x fmt -x run