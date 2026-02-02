set -e 

SCRIPT_ORIGIN=$(pwd)
REPO_ROOT=$(git rev-parse --show-toplevel)

cd $REPO_ROOT

command_exists() {
    command -v "$1"
}

if command_exists rustc && command_exists cargo; then
    echo "Rust is installed."
else
    echo "Installing rust"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

if npm list --depth=0 tsx; then
    echo "Found tsx installation"
else
    # Node Dependencies installation
    if [ -f "${REPO_ROOT}/yarn.lock" ]; then
        yarn global add tsx
    elif [ -f "${REPO_ROOT}/pnpm-lock.yaml" ]; then
        pnpm add -g tsx
    elif [ -f "${REPO_ROOT}/package-lock.json" ]; then
        npm i -g tsx
    else
        echo "No node.js package manager detected"
        echo "Run npm init to create your node.js project"
    fi
fi


echo "Installing Repokit CLI"

cd "$SCRIPT_ORIGIN"

echo "Compiling from $SCRIPT_ORIGIN"

. "$HOME/.cargo/env"
RUSTFLAGS="-Awarnings" cargo build --release
cargo install --path .

cd "$REPO_ROOT"
repokit