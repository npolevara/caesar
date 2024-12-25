## A simple Caesar cipher implementation on Rust

### build & run: 
```console
cargo run -r

```
### run cipher: 
- -p prompt
- -s shift 0-25
- -d decrypt [optional]
```console
cargo run -- -p "hello world" -s 3

// encrypt:
target/release/caesar -p"styled things" -s3
=> Encrypting from "styled things" to:  vwbohg wklqjv

// decrypt:
target/release/caesar -p"vwbohg wklqjv" -s3 -d
=> Decrypting from "vwbohg wklqjv" to:  styled things
```
