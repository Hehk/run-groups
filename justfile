dev:
  #!/bin/bash
  trunk serve "web/index.html" 2>&1 | while read -r line; do
    echo "$line"

    if [[ "$line" == *"server listening at"* ]]; then
      URL=$(echo "$line" | awk '{print $NF}')
      open -a "Google Chrome" "${URL}"
    fi
  done