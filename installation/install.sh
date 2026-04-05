CURRENT_VERSION="3.1.0"
CWD=$(pwd)

REPLACEMENT="/node_modules"
FALLBACK_ROOT="${CWD%${REPLACEMENT}*}"

GIT_ROOT=$(git rev-parse --show-toplevel)
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

OLD_SETTINGS_FILE=".repokit"
CACHED_THEME=""

if [ -f $OLD_SETTINGS_FILE ]; then
    {
        read -r
        read -r CACHED_THEME
    } < "$OLD_SETTINGS_FILE"
    rm "$OLD_SETTINGS_FILE"
fi

CACHE_DIR=".repokit"
VERSION_FILE=".repokit_version"
SETTINGS_FILE=".repokit_settings"

mkdir -p "$CACHE_DIR"

cd "$CACHE_DIR"

if [ -f $VERSION_FILE ]; then
    read -r FIRST_LINE < "$VERSION_FILE"
    if [ "$FIRST_LINE" == "$CURRENT_VERSION" ]; then
        exit 0;    
    fi
else
    touch "$VERSION_FILE"
fi 

printf "$CURRENT_VERSION\n" > "$VERSION_FILE"

cd $REPO_ROOT

ROOT_COMMIT=$(git rev-list --parents HEAD | tail -1) || ROOT_COMMIT=""

if [ -n "$ROOT_COMMIT" ] && [ -n "$CACHED_THEME" ]; then
    cd
    cd "$CACHE_DIR"
    mkdir -p "$ROOT_COMMIT"
    cd "$ROOT_COMMIT"
    touch "$SETTINGS_FILE"
    printf "$CACHED_THEME\n" > "$SETTINGS_FILE"
fi

cd $CWD

echo "Compiling from $CWD"

. "$HOME/.cargo/env"
RUSTFLAGS="-Awarnings" cargo build --release
cargo install --path .