all:
	@cargo build
update:
	@cp ../proton-cli/target/debug/proton ./proton_cli
	@cargo run update-data asdf
run:
	@cp ../proton-cli/target/debug/proton ./proton_cli
	@cargo run run-show asdf "/dev/ttyUSB0"
allOn:
	@cargo run allOn "/dev/ttyUSB0"
allOff:
	@cargo run allOff "/dev/ttyUSB0"

