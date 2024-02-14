.POSIX:
.PHONY: *

default: build

build:
	nix build

dev:
	NTFY_TOPIC="webhook-transformer" cargo run -- \
		--upstream-host https://ntfy.sh \
		--config examples/alertmanager-to-ntfy/config.jsonnet

test:
	cargo test

docker:
	nix build '.#dockerImage'
	docker load < ./result
