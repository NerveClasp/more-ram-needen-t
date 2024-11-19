# more ram needen't

Tool to handle large files

## Install Rust

See details [here](https://www.rust-lang.org/tools/install)
or run

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Run using cargo

```sh
cargo run -- -f ~/Path/To/Your/File
```

for example

```sh
cargo run -- -f ./src/testfile.txt
```

## Or build and run

```sh
cargo build && cd ./target/debug/
```

and finally

```sh
./more-ram-needen-t -f ~/Path/To/Your/File
```

### Copy built file

You can copy the binary somewhere into your system to be able to run it anytime

```sh
sudo cp more-ram-needen-t /usr/local/bin/
```
