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

Office PC:
```sh
export RUSTC=/local/home/zixliu/rustc-tooling/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
export RUSTDOC=/local/home/zixliu/rustc-tooling/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc

```


4. usage
```sh
cargo doc
```

5. Project structure

Main entrance: `./src/librustdoc/lib.rs::tooling_main_args()`

Processing logic: `./src/librustdoc/tooling/mod.rs`