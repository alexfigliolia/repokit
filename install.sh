set -e 

REPO_ROOT=$(git rev-parse --show-toplevel)

cd $REPO_ROOT

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

if command_exists rustc && command_exists cargo; then
    echo "Rust is installed."
else
    echo "Installing rust"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

if npm list --depth=0 tsx >/dev/null 2>&1; then
    echo "Found tsx installation"
else
    # Node Dependencies installation
    if [ -f "${REPO_ROOT}/yarn.lock" ]; then
        yarn add -D tsx
    elif [ -f "${REPO_ROOT}/pnpm-lock.yaml" ]; then
        pnpm add -D tsx
    elif [ -f "${REPO_ROOT}/package-lock.json" ]; then
        npm i -D tsx
    else
        echo "No node.js package manager detected"
        echo "Run npm init to create your node.js project"
    fi
fi


echo "Installing devkit CLI"

SCRIPT_DIR=$(dirname "$(readlink -f "$0")")
cd "$SCRIPT_DIR"

. "$HOME/.cargo/env"
RUSTFLAGS="-Awarnings" cargo build --release > /dev/null
cargo install --path . > /dev/null
devkit