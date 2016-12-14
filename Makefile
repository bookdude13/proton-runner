all:
	@cargo build
copy:
	@cp ../proton-cli/target/debug/proton ./proton_cli
update:
	@cp ../proton-cli/target/debug/proton ./proton_cli
	@cargo run update-data show2016
run:
	@cargo run run-show show2016 "/dev/ttyUSB0"
run1:
	@cargo run run-show show2016 "/dev/ttyUSB1"
test:
	@cargo test
on:
	@cargo run allon "/dev/ttyUSB0"
off:
	@cargo run alloff "/dev/ttyUSB0"

