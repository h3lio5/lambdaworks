test:
	cargo test

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

docker-shell:
	docker build -t rust-curves .
	docker run -it rust-curves bash

nix-shell:
	nix-shell

benchmarks:
	cargo criterion --bench all_benchmarks

# BENCHMARK should be one of the [[bench]] names in Cargo.toml
benchmark:
	cargo criterion --bench ${BENCHMARK}

benchmarks_with_flamegraphs:
	cargo bench --features benchmark_flamegraph --bench all_benchmarks -- --profile-time=5

# BENCHMARK should be one of the [[bench]] names in Cargo.toml
benchmark_with_flamegraphs:
	cargo bench --features benchmark_flamegraph --bench ${BENCHMARK} -- --profile-time=5
