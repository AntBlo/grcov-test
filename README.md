
### Setup
`cargo install grcov`  
`rustup component add llvm-tools-preview`

### generate
`LLVM_PROFILE_FILE="./coverage/%p-%m.profraw" cargo test --workspace --no-fail-fast`  
`grcov . --binary-path ./target/debug/ -s . -t lcov --branch --ignore-not-existing -o ./coverage/lcov.info`  
`grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/html`
