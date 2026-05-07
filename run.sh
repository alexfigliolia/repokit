#!/bin/sh

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)

for dir in $SCRIPT_DIR/../@repokit/*/; do
    case "$dir" in
      *@repokit/core/) continue ;;
      *)
        if [[ -f "$dir/repokit" ]]; then
          "$dir/repokit" "$@"
          exit $?
        fi
        if [[ -f "$dir/repokit.exe" ]]; then
          "$dir/repokit.exe" "$@"
          exit $?
        fi
      ;;
  esac
done

RUSTFLAGS="-A warnings" cargo run --release