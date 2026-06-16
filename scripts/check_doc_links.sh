#!/usr/bin/env sh
set -eu

missing=0
for file in README.md SECURITY.md CHANGELOG.md ROADMAP.md docs/*.md release-notes/*.md; do
    [ -f "$file" ] || continue
    sed -n 's/.*](\([^)]*\.md\)).*/\1/p' "$file" | while IFS= read -r link; do
        case "$link" in
            http://*|https://*) continue ;;
        esac
        base="$(dirname "$file")"
        target="$base/$link"
        if [ ! -f "$target" ]; then
            echo "missing doc link in $file: $link" >&2
            missing=1
        fi
    done
done

exit "$missing"
