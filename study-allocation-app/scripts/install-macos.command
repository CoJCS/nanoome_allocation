#!/bin/bash
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
APP="$(find "$DIR" -maxdepth 1 -name '*.app' -type d | head -1)"
DEST="/Applications/$(basename "${APP:-}")"

if [[ -z "$APP" ]]; then
  osascript -e 'display alert "설치 실패" message "같은 폴더에 .app 파일이 없습니다." as critical'
  exit 1
fi

echo "설치 중: $(basename "$APP")"
rm -rf "$DEST"
ditto --norsrc "$APP" "$DEST"
codesign --force --deep --sign - "$DEST"
open "$DEST"

echo "Applications에 설치했습니다."
