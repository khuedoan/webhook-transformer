.POSIX:
.PHONY: default build oci-image

default: build

build:
	nix build

oci-image:
	nix build '.#dockerImage'
	docker load < ./result
