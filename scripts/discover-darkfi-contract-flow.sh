#!/usr/bin/env bash
set -euo pipefail

DARKFI_ROOT="${DARKFI_ROOT:-/home/darkfi/darkfi-lab/darkfi}"
OUT_DIR="${OUT_DIR:-/home/darkfi/darkfi-lab/faucet/runtime/logs/contract-discovery}"

[[ -d "$DARKFI_ROOT" ]] || { echo "DarkFi root not found: $DARKFI_ROOT" >&2; exit 1; }
mkdir -p "$OUT_DIR"
chmod 700 "$OUT_DIR"

{
  echo "# DarkFi contract discovery"
  echo
  echo "DARKFI_ROOT=$DARKFI_ROOT"
  echo "generated_at=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo
  echo "## Candidate contract directories"
  find "$DARKFI_ROOT" \( -path "$DARKFI_ROOT/.git" -o -path "$DARKFI_ROOT/target" \) -prune -o \
    -maxdepth 5 -type d \( -iname '*contract*' -o -iname '*membership*' -o -iname '*example*' \) -print | sort
  echo
  echo "## Candidate command references"
  grep -RInE 'deploy|contract|membership|wasm|call|mint|withdraw|transfer|token' "$DARKFI_ROOT" \
    --exclude-dir='.git' --exclude-dir='target' --exclude-dir='node_modules' \
    --include='*.md' --include='*.sh' --include='Makefile' --include='*.toml' --include='*.rs' \
    2>/dev/null | head -500 || true
} > "$OUT_DIR/discovery.md"

echo "Wrote $OUT_DIR/discovery.md"
