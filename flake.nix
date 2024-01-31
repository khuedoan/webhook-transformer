{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = craneLib.cleanCargoSource ./.;
        nativeBuildInputs = with pkgs; [
          rustToolchain
        ];
        buildInputs = with pkgs; [
          # TODO?
        ];
        commonArgs = {
          inherit src buildInputs nativeBuildInputs;
          doCheck = false;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        bin = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
        dockerImage = pkgs.dockerTools.buildImage {
          name = "webhook-transformer";
          tag = "latest";
          copyToRoot = [
            bin
          ];
          config = {
            entrypoint = [ "${bin}/bin/webhook-transformer" ];
          };
        };
      in
      with pkgs;
      {
        packages = {
          inherit bin dockerImage;
          default = bin;
        };
        devShells.default = mkShell {
          inputsFrom = [
            bin
          ];
        };
      }
    );
}
