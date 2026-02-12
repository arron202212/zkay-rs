for f in util/test/*.rs; do mv "$f" "$(echo "$f" | perl -pe 's/([a-z0-9])([A-Z])/$1_$2/g' | tr '[:upper:]' '[:lower:]')"; done
