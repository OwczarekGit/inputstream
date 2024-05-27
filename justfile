default:

clean:
    cargo clean

linux:
    cargo build --release

windows:
    RUSTFLAGS='-L SDL2/' cross build --release --target x86_64-pc-windows-gnu
