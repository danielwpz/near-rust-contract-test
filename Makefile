RFLAGS="-C link-arg=-s"

lottery: contracts/lottery
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p lottery --target wasm32-unknown-unknown --release
	mkdir -p res
	cp target/wasm32-unknown-unknown/release/lottery.wasm ./res/lottery.wasm

mock-ft: contracts/mock-ft
	rustup target add wasm32-unknown-unknown
	RUSTFLAGS=$(RFLAGS) cargo build -p mock-ft --target wasm32-unknown-unknown --release
	mkdir -p res
	cp target/wasm32-unknown-unknown/release/mock_ft.wasm ./res/mock_ft.wasm

test: test-unit test-ava

test-unit:
	cargo test

test-ava: lottery mock-ft
	npx near-workspaces-ava --timeout=2m /Users/daniel/Programs/dapps/near/test-course/__tests__/lottery.ava.ts --verbose
