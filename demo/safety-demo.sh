#!/usr/bin/env bash
# Reproducible ~30s safety demo for CX Linux (Ubuntu 24.04).
# Uses CX_ASK_MOCK_RESPONSE (offline, labeled) — no paid API key.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MARKER="${HOME}/.cx/demo-risk-marker"
MOCK_JSON=$(cat <<'EOF'
{
  "status": "success",
  "source": "mock-offline",
  "message": "[mock-offline] Simulated AI suggestion for demo only.",
  "command": "touch \"$HOME/.cx/demo-risk-marker\"",
  "undo": "rm -f \"$HOME/.cx/demo-risk-marker\""
}
EOF
)

export CX_ASK_MOCK_RESPONSE="$MOCK_JSON"

echo "=== CX safety demo (offline mock) ==="
rm -f "$MARKER"
rm -rf "${HOME}/.cx/audit.log" "${HOME}/.cx/transactions.jsonl" 2>/dev/null || true

if ! command -v cx-terminal >/dev/null 2>&1; then
  echo "Building cx-terminal from source (first run may take several minutes)..."
  cd "$ROOT"
  export CXX="${CXX:-g++-12}"
  export CC="${CC:-gcc-12}"
  export RUSTFLAGS="${RUSTFLAGS:--C linker=g++-12}"
  rustup run stable cargo build -p cx-terminal --release --locked
  export PATH="$ROOT/target/release:$PATH"
fi

CX="${CX_BIN:-cx-terminal}"
if command -v cx >/dev/null 2>&1; then
  CX=cx
fi

echo ""
echo "--- 1) Natural language request (default: dry-run only) ---"
"$CX" ask "create a demo risk marker file" </dev/null || true
test ! -f "$MARKER" || { echo "FAIL: marker created without --execute"; exit 1; }

echo ""
echo "--- 2) Execute with --execute -y ---"
"$CX" ask --execute -y "create a demo risk marker file"
test -f "$MARKER" || { echo "FAIL: marker missing after execute"; exit 1; }

echo ""
echo "--- 3) Audit log (last entries) ---"
"$CX" audit --tail 3

echo ""
echo "--- 4) Undo ---"
"$CX" undo
test ! -f "$MARKER" || { echo "FAIL: marker still present after undo"; exit 1; }

echo ""
echo "=== Demo complete ==="
