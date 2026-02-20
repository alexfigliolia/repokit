
CWD=$(pwd)

REPLACEMENT="/node_modules"
FALLBACK_ROOT="${CWD%${REPLACEMENT}*}"

SCRIPT_ORIGIN=$(pwd)
GIT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null)
REPO_ROOT=${GIT_ROOT:-$FALLBACK_ROOT}


cd $REPO_ROOT

command_exists() {
    command -v "$1" > /dev/null 2>&1
}

if command_exists rustc && command_exists cargo; then
    echo "Rust is installed."
else
    echo "Installing rust"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

echo "Installing Repokit CLI"

cd $SCRIPT_ORIGIN

echo "Compiling from $SCRIPT_ORIGIN"

. "$HOME/.cargo/env"
RUSTFLAGS="-Awarnings" cargo build --release
cargo install --path .

cd $REPO_ROOT
repokit