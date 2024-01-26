.POSIX:
.PHONY: *

default: build

build:
	nix build

dev:
	cargo run -- --config examples/basic/config.jsonnet

test:
	cargo test

docker:
	nix build '.#dockerImage'
	docker load < ./result
