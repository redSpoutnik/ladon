SCRIPT_PATH="$(pwd)/$(dirname $0)"
cd $SCRIPT_PATH && git pull && cargo build --release