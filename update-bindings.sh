#! /bin/sh -ea

BASE_DIR=$(dirname $0)
BINDINGS_DEST="$BASE_DIR/src/bindings.rs"

if ! which rustfmt 2>&1 1>/dev/null; then
  echo "installing rustfmt"
  rustup component add rustfmt-preview
fi

if ! which bindgen 2>&1 1>/dev/null; then
  echo "installing bindgen"
  cargo install bindgen
fi

# update submodule
echo "updating xproto submodule"
cd "$BASE_DIR/xproto"
git fetch -q
git checkout -q origin/master
cd ../

echo "generating bindings at $BINDINGS_DEST"

# NOTE:
# Requires rust-bindgen#1446 for `--disable-untagged-union`.
# https://github.com/rust-lang/rust-bindgen/pull/1446
bindgen "$BASE_DIR/xproto/Xproto.h" \
        -o "$BINDINGS_DEST" \
        --rustfmt-bindings \
        --disable-untagged-union

sed -i 's/derive(/derive(Protocol, /g' "$BINDINGS_DEST"

sed -i 's/pub struct __BindgenUnionField/#[derive(Protocol)]\npub struct __BindgenUnionField/g' "$BINDINGS_DEST"


