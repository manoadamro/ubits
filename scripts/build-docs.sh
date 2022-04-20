#!/usr/bin/env sh

BASE=${1:-$(pwd)};
DOC_PATH="${BASE}/docs";
INDEX_PATH="${DOC_PATH}/index.html"

rm -rf "${DOC_PATH}";
mkdir "${DOC_PATH}";
echo "<meta http-equiv=\"refresh\" content=\"0; url=doc/josh\">" > "${INDEX_PATH}";
cargo doc --workspace --no-deps --target-dir "${DOC_PATH}";