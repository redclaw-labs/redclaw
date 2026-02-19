#!/usr/bin/env bash
set -euo pipefail

# Brand scan: detect legacy/forbidden brand strings in the repo.
# IMPORTANT: LICENSE is always excluded because it must preserve original legal attribution.

ROOT="${1:-.}"

# Keep this list tight and explicit.
PATTERN='zeroclaw|ZeroClaw|ZEROCLAW|theonlyhennygod'

if grep -RIn -E "$PATTERN" "$ROOT" \
  --exclude-dir=target \
  --exclude-dir=.git \
  --exclude=LICENSE \
  --include='*.rs' \
  --include='*.toml' \
  --include='*.md'; then
  echo "\n❌ Brand scan failed: forbidden strings found (LICENSE excluded)." >&2
  exit 1
fi

echo "✅ Brand scan passed (LICENSE excluded)."
