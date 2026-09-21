def_env:
	export CARGO_HOME="./.cargo_env"
watch:
	bacon

channels:
	cargo run --bin channels
