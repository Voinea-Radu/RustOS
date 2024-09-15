curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup target add thumbv7em-none-eabihf

rustup component add rust-src
rustup target add x86_64-unknown-linux-gnu
rustup target add nightly-x86_64-unknown-linux-gnu

cargo install bootimage
rustup component add llvm-tools-preview

apt install qemu-system-x86