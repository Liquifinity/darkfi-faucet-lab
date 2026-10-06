#!/usr/bin/env bash
set -euo pipefail

DRK_BIN="${DRK_BIN:-/home/darkfi/darkfi-lab/darkfi/drk}"
DRK_MAIN_CONFIG="${DRK_MAIN_CONFIG:?Set this in the private operator environment}"
DRK_TOKEN_ID="${DRK_TOKEN_ID:?Set this in the private operator environment}"
FAUCET_ADDRESS="${FAUCET_ADDRESS:?Set this in the private operator environment}"
DAILY_TOP_UP_AMOUNT="${DAILY_TOP_UP_AMOUNT:-0.33}"
TX_DIR="${TX_DIR:-/home/darkfi/darkfi-lab/faucet/runtime/state/tx}"

mode="${1:---dry-run}"
[[ "$mode" == "--dry-run" || "$mode" == "--broadcast" ]] || { echo "Usage: $0 [--dry-run|--broadcast]" >&2; exit 2; }
mkdir -p "$TX_DIR"
chmod 700 "$TX_DIR"
tx_path="$TX_DIR/faucet-topup-$(date -u +%Y%m%dT%H%M%SZ).tx"

"$DRK_BIN" -c "$DRK_MAIN_CONFIG" transfer "$DAILY_TOP_UP_AMOUNT" "$DRK_TOKEN_ID" "$FAUCET_ADDRESS" > "$tx_path"
chmod 600 "$tx_path"
"$DRK_BIN" -c "$DRK_MAIN_CONFIG" inspect < "$tx_path"

if [[ "$mode" == "--broadcast" ]]; then
  "$DRK_BIN" -c "$DRK_MAIN_CONFIG" broadcast < "$tx_path"
  "$DRK_BIN" -c "$DRK_MAIN_CONFIG" spend < "$tx_path" || true
else
  echo "Dry run only; transaction was not broadcast."
fi
