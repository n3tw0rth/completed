set -e
cargo build --release
sudo cp ./target/release/hark  /usr/bin/
