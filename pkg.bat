echo y| rmdir "./pkg/Real" /s
mkdir "./pkg/Real"
echo d| xcopy "./assets" "./pkg/Real/assets" /e
cargo build --release
cd target\release && copy "real.exe" "../../pkg/Real"
cd ..\..