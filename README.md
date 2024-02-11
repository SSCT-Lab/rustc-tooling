1. vscode + rust-analyzer
```sh
./x.py setup vscode
```

2. build
```sh
python3 ./x.py build --stage 2
```

3. tool
```sh
export RUSTC=/home/cici/rustc-tooling/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
export RUSTDOC=/home/cici/rustc-tooling/build/x86_64-unknown-linux-gnu/stage2/bin/tooling
```

4. usage
```sh
cargo doc
```