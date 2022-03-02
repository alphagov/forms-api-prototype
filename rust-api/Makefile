##
# Rust API Spike
##

run: deps
	docker-compose up -d
	cargo watch -c -x run

deps:
	@echo "Checking dependencies..."
	@which docker-compose || echo "Please install docker-compose"
	@which cargo || echo "Please install cargo from rustup.rs"
	@cargo watch --version || cargo install cargo-watch -q
