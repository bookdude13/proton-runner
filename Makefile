all:
	@cargo build
copy_proton:
	@cp ../proton-cli/target/debug/proton ./proton_cli

