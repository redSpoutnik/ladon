SCRIPT_PATH="$(pwd)/$(dirname $0)"
BIN_PATH=$(echo "$PATH" | tr ':' '\n' | head -n 1)
cd $SCRIPT_PATH && cargo build --release && ln -s "$SCRIPT_PATH/target/release/ladon" "$BIN_PATH/ladon"