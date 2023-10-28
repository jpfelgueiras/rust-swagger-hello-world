# Steps on os to prepare

```shell 
sudo apt install cargo rust-src
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | 
```

Add this line to .zshrc

```shell
export PATH="$PATH:$HOME/.cargo/bin"
```

```shell
cargo binstall cargo-watch
```

# Step to run

```shell
cargo watch -x run
```