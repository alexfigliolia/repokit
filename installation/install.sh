CURRENT_VERSION="4.0.4"
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

OLD_CACHE_FILE=".repokit";
CACHE_DIRECTORY=".repokit_cache";
VERSION_FILE=".version"
SETTINGS_FILE=".settings"
REPO_CACHE_DIRECTORY="$CACHE_DIRECTORY/$ROOT_COMMIT"

OLD_SCHEMA_CACHED_THEME=""
OLD_SCHEMA_CACHED_VERSION="$CURRENT_VERSION"

if [ -f "$OLD_CACHE_FILE" ]; then
    {
        read -r OLD_SCHEMA_CACHED_VERSION
        read -r OLD_SCHEMA_CACHED_THEME
    } < "$OLD_CACHE_FILE"
    if [ -n "$ROOT_COMMIT" ] && [ -f "$REPO_CACHE_DIRECTORY/$SETTINGS_FILE" ]; then
        OLD_SCHEMA_CACHED_THEME=""
    fi
fi 

if [ ! -d "$CACHE_DIRECTORY" ]; then 
    mkdir "$CACHE_DIRECTORY"    
fi 

cd "$CACHE_DIRECTORY"

if [ -n "$ROOT_COMMIT" ]; then
    if [ ! -d "$ROOT_COMMIT" ]; then
        mkdir "$ROOT_COMMIT"
    fi
    cd $ROOT_COMMIT;
    if [ ! -f "$SETTINGS_FILE" ]; then
        touch "$SETTINGS_FILE"
    fi
    if [ -n "$OLD_SCHEMA_CACHED_THEME" ]; then
        echo "$OLD_SCHEMA_CACHED_THEME\n" > "$SETTINGS_FILE"
    fi
    cd "../"
fi

if [ -f $VERSION_FILE ]; then
    read -r CACHED_VERSION < "$VERSION_FILE"
    if [ "$CACHED_VERSION" == "$CURRENT_VERSION" ] && [ "$CACHED_VERSION" == "$OLD_SCHEMA_CACHED_VERSION" ]; then
        exit 0;    
    fi
else 
    touch "$VERSION_FILE"
fi 

echo "$CURRENT_VERSION\n" > "$VERSION_FILE"

cd ../

if [ -f "$OLD_CACHE_FILE" ]; then
    TEMP_FILE=".repokit_tmp";
    printf "$CURRENT_VERSION\n" > "$TEMP_FILE"
    tail +2 "$OLD_CACHE_FILE" >> "$TEMP_FILE"
    mv "$TEMP_FILE" "$OLD_CACHE_FILE"
fi

cd $CWD

echo "Compiling from $CWD"

. "$HOME/.cargo/env"
RUSTFLAGS="-Awarnings" cargo build --release
cargo install --path .