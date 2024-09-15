cargo build --release
rm -rf ./bin
mkdir bin
cp ./target/x86_64-unknown-none/release/bobos ./bin/bobos
cargo bootimage
rm -rf ./bobos
mkdir bobos
cp ./target/target/debug/bootimage-bobos.bin bobos/bobos.bin