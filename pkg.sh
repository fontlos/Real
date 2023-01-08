rm -rf "./pkg/Real"
mkdir "./pkg"
mkdir "./pkg/Real"
cp -r "./assets" "./pkg/Real/assets"
cargo build --release
cp "./target/release/real" "./pkg/Real"