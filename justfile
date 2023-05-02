dev:
  #!/bin/bash
  set -euo pipefail

  processes=$((lsof -i :3000 -i :3001 || true) | (grep LISTEN || true))
  if [[ -n "$processes" ]]; then
    echo "Killing processes listening on ports 3000 and 3001..."
    echo "$processes" | awk '{print $2}' | xargs kill -9
  fi

  cargo leptos watch 2>&1 | while read -r line; do
    echo "$line"

    if [[ "$line" == *"watching folders"* ]]; then
      URL=$(cat web/Cargo.toml | grep -m 1 'site-addr' | awk '{print substr($NF, 2, length($NF)-2)}')
      open -a "Google Chrome" "http://${URL}"
    fi
  done