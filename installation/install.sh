CURRENT_VERSION="3.0.7"
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

ROOT_COMMIT=$(git rev-list --parents HEAD | tail -1) || ROOT_COMMIT=""

cd

CACHE_FILE=".repokit";
CACHE_DIRECTORY=".repokit_cache";
NEW_SETTINGS_FILE=".settings"
NEW_VERSION_FILE=".version"
REPO_CACHE_DIRECTORY="$CACHE_DIRECTORY/$ROOT_COMMIT"
LAST_THEME_USED=""
NEW_CACHE_PATH_VERSION="$CURRENT_VERSION"
BACK_PORTING=0

if [ -n "$ROOT_COMMIT" ] && [ -f "$REPO_CACHE_DIRECTORY/$NEW_SETTINGS_FILE" ]; then
    BACK_PORTING=1
    cd "$REPO_CACHE_DIRECTORY"
    read -r LAST_THEME_USED < "$NEW_SETTINGS_FILE"
    rm "$NEW_SETTINGS_FILE"
    cd
fi

if [ -f "$CACHE_DIRECTORY/$NEW_VERSION_FILE" ]; then
    read -r NEW_CACHE_PATH_VERSION < "$CACHE_DIRECTORY/$NEW_VERSION_FILE"
fi

if [ ! -f "$CACHE_FILE" ]; then
    touch "$CACHE_FILE"
elif [ "$BACK_PORTING" == 0 ]; then
    read -r LAST_VERSION_USED < "$CACHE_FILE"
    if [ "$LAST_VERSION_USED" == "$CURRENT_VERSION" ] && [ "$LAST_VERSION_USED" == "$NEW_CACHE_PATH_VERSION" ]; then
        exit 0
    fi
fi

if [ -n "$LAST_THEME_USED" ]; then 
    echo "$CURRENT_VERSION\n$LAST_THEME_USED\n" > "$CACHE_FILE"
else
    TEMP_FILE=".repokit_tmp";
    printf "$CURRENT_VERSION\n" > "$TEMP_FILE"
    tail +2 "$CACHE_FILE" >> "$TEMP_FILE"
    mv "$TEMP_FILE" "$CACHE_FILE"
fi

if [ -f "$CACHE_DIRECTORY/$NEW_VERSION_FILE" ]; then
    echo "$CURRENT_VERSION\n" > "$CACHE_DIRECTORY/$NEW_VERSION_FILE"
fi


cd $CWD

echo "Compiling from $CWD"

. "$HOME/.cargo/env"
RUSTFLAGS="-Awarnings" cargo build --release
cargo install --path .