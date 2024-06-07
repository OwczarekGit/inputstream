default:

all: linux windows

clean:
    cargo clean

linux:
    cargo build --release --target=x86_64-unknown-linux-gnu

windows:
    RUSTFLAGS='-L SDL2/' cross build --release --target x86_64-pc-windows-gnu
