#!/usr/bin/env bash
# Build the WASM module + JS bindings into ./pkg and print how to serve it.
set -euo pipefail
cd "$(dirname "$0")"

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "wasm-pack not found. Install it with:  cargo install wasm-pack" >&2
  exit 1
fi

wasm-pack build --target web --release --out-dir pkg

cat <<EOF

✅ Built ./pkg

Serve locally (needs a real HTTP server — file:// won't load the wasm module):
    python3 -m http.server -d "$(pwd)" 8080

Then open:
    http://localhost:8080/
EOF
