#
# Makefile
# @author Evgeny Ukhanov <mrlsd@ya.ru>
#

check:
	@cargo check

run:
	@cargo run

test:
	@cargo test

build:
	@cargo build

release:
	@cargo build --release
