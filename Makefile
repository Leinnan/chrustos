all:
	bootimage run --target=chrustos.json

tree:
	tree -A -I target

clean:
	rm -f Cargo.lock
	rm -f chrustos.bin
	cargo clean
