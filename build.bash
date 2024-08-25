
# To prep for build
# rustup target add x86_64-pc-windows-gnu
# sudo apt-get install mingw-w64

cargo build --release --target x86_64-pc-windows-gnu

# Target is located at ./target/x86_64-pc-windows-gnu/release/runling-run-edit.exe