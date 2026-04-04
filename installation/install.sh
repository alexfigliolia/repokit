CURRENT_VERSION="3.0.3"
CWD=$(pwd)

REPLACEMENT="/node_modules"
FALLBACK_ROOT="${CWD%${REPLACEMENT}*}"

GIT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null)
REPO_ROOT=${GIT_ROOT:-$FALLBACK_ROOT}

if [[ "$CWD" != *"$REPLACEMENT"* ]]; then
    exit 0;
fi


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

cd

CACHE_DIR=".repokit"
DOT_FILE=".repokit_version"

mkdir "$CACHE_DIR"

cd "$CACHE_DIR"

if [ -f $DOT_FILE ]; then
    read -r first_line < ".repokit_version"
    if [ "$first_line" = "$CURRENT_VERSION" ]; then
        exit 0;    
    fi
fi

touch "$DOT_FILE"

printf "$CURRENT_VERSION\n" > "$DOT_FILE"

cd $CWD

echo "Compiling from $CWD"

. "$HOME/.cargo/env"
RUSTFLAGS="-Awarnings" cargo build --release
cargo install --path .