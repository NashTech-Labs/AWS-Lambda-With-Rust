# AWS-Lambda-With-Rust

Here we have written a Rust code which can run in the AWS Lambda.


## Setup:

Install the `musl-tools`:

```sh
sudo apt install musl-tools
```

```sh
rustup target add x86_64-unknown-linux-musl         # First time only
```

## Common building steps:

To Build the rust code run this command:
```sh
cargo build --release --target x86_64-unknown-linux-musl
```

Then we have to make the zip file of this code to run it in the AWS Lambda.
```sh
zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
```
Now you can use this zip file  in your AWS Lambda function.
