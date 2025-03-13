set -e
cargo build --release
sudo cp ./target/release/completed  /usr/bin/
