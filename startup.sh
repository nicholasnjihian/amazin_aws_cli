#!/bin/bash

echo "If cargo is not installed it will be installed in '$HOME'" 
echo ""

CURRENT_DIR=.
CARGO_DIR=$HOME/.cargo/bin 
RUSTUP_DIR=$HOME/.cargo/bin/rustup

#Check whether '~/.cargo' dir exists and rustup is installed
if [[ -d "$CARGO_DIR" && -x "$RUSTUP_DIR" ]]; then
	export PATH="$CARGO_DIR:$PATH"
	echo "cargo and rustup added to path...."
	echo "Current cargo version=="
	cargo --version
	echo ""
	echo "Current rustup version =="
	rustup --version
	echo ""
	echo "rustup toolchain management tool for the Rust Programming Language is installed"
	echo "We're good to go."
	echo "Let's now build our package. You can enter --help after running the script"
	echo ""
	cargo run
	
else 
	#Install rustup
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

