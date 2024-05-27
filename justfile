default:

clean:
    cargo clean

linux:
    cargo build --release

windows:
    cross build --release --target x86_64-pc-windows-gnu
