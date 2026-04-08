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

ROOT_COMMIT=$(git rev-list --parents HEAD | tail -1) || ROOT_COMMIT=""

cd

CACHE_FILE_OR_DIRECTORY=".repokit";
NEW_VERSION_FILE=".repokit_version"
NEW_SETTINGS_FILE=".repokit_settings"
REPO_CACHE_DIRECTORY="$CACHE_FILE_OR_DIRECTORY/$ROOT_COMMIT";

CACHED_THEME=""

if [ -d "$CACHE_FILE_OR_DIRECTORY" ]; then
    if [ -n "$ROOT_COMMIT" ] && [ -d "$REPO_CACHE_DIRECTORY" ]; then
        {
            read -r CACHED_THEME
        } < "$REPO_CACHE_DIRECTORY/$NEW_SETTINGS_FILE"
    fi
    rm -rf "$CACHE_FILE_OR_DIRECTORY"
elif [ -f "$CACHE_FILE_OR_DIRECTORY" ]; then
    read -r PREVIOUS_VERSION < "$CACHE_FILE_OR_DIRECTORY"
    if [ "$PREVIOUS_VERSION" == "$CURRENT_VERSION" ]; then
        exit 0;    
    fi
else
    touch "$VERSION_FILE"
fi 

touch "$CACHE_FILE_OR_DIRECTORY"

if [ -n "$CACHED_THEME" ]; then
    printf "$CURRENT_VERSION\n$CACHED_THEME\n" > "$CACHE_FILE_OR_DIRECTORY"
else 
    TEMP_FILE=".repokit_tmp";
    printf "$CURRENT_VERSION\n" > "$TEMP_FILE"
    tail +2 "$CACHE_FILE_OR_DIRECTORY" >> "$TEMP_FILE"
    mv "$TEMP_FILE" "$CACHE_FILE_OR_DIRECTORY"
fi

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