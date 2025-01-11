set -e

cargo build --release

sudo cp ./target/release/completion-notifier  /usr/bin/
