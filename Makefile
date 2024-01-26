.POSIX:
.PHONY: *

default: build

build:
	nix build

dev:
	nix run

test:
	cargo test

docker:
	nix build '.#dockerImage'
	docker load < ./result
