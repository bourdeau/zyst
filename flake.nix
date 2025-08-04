{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";

    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-manifest = {
      url = "https://static.rust-lang.org/dist/channel-rust-1.88.0.toml";
      flake = false;
    };
  };

  outputs = inputs@{ self, nixpkgs, systems, flake-parts, crane, fenix, rust-manifest, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import systems;

      perSystem = { self', system, pkgs, lib, ... }:
        let
          cargoToml = lib.importTOML ./Cargo.toml;
          pname = cargoToml.package.name or "zyst";
          version = cargoToml.package.version or "1.0.3";

          rustToolchain = (fenix.packages.${system}.fromManifestFile rust-manifest).defaultToolchain;
          craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              ./rustfmt.toml
              ./benches
              ./src
              ./tests
            ];
          };

          commonArgs = {
            inherit src pname version;
            buildInputs = [];
            nativeBuildInputs = [ pkgs.pkg-config ];
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          myPackage = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;
            cargoTestExtraArgs = "--test mod ut";
          });
        in
        {
          packages.default = myPackage;

          checks = {
            fmt = craneLib.cargoFmt commonArgs;
            clippy = craneLib.cargoClippy (commonArgs // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--workspace -- --deny warnings";
            });
            test = craneLib.cargoTest (commonArgs // {
              inherit cargoArtifacts;
              cargoTestExtraArgs = "--test mod ut";
            });
          };

          devShells.default = craneLib.devShell {
            checks = self'.checks;
            buildInputs = [
              myPackage
              pkgs.rust-analyzer
            ];
          };

          formatter = pkgs.nixpkgs-fmt;
        };
    };
}
