## Building for Alpine (musl)

`rustup target add x86_64-unknown-linux-musl`

then

`cargo build -r --target=x86_64-unknown-linux-musl`

CI/CD

sudo apt-get install -y --no-install-recommends musl-tools

check
https://github.com/zellij-org/zellij/blob/main/.github/workflows/release.yml

## Watch CI process

gh run watch

## Review processed CI workflows

gh run view --log-failed

## Rerunning failed CI

gh run rerun --failed
