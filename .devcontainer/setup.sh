## update and install some things we should probably have
apt-get update
apt-get install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  zsh \
  vim \
  build-essential \
  openssl
  
## update and install 2nd level of packages
apt-get install -y pkg-config libssl-dev

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y
export PATH=/root/.cargo/bin:$PATH
rustup install nightly
rustup component add rustfmt
rustup component add rustfmt --toolchain nightly
rustup component add clippy 
rustup component add clippy --toolchain nightly

cargo install cargo-expand
cargo install cargo-edit
