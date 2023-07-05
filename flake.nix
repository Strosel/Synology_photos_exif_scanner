{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/23.05";
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
        flake-utils.follows = "flake-utils";
        rust-overlay.follows = "rust-overlay";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };


        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = craneLib.cleanCargoSource ./.;
        nativeBuildInputs = with pkgs; [ rustToolchain pkg-config ];
        buildInputs = with pkgs; [ exiftool ];

        # because we'll use it for both `cargoArtifacts` and `bin`
        commonArgs = {
          inherit src buildInputs nativeBuildInputs;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        bin = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

        dockerImage = pkgs.dockerTools.buildImage {
          name = "synology_photos_exif_scanner";
          tag = "latest";
          copyToRoot = [ bin ];
          config = {
            Cmd = [ "${bin}/bin/synology_photos_exif_scanner" ];
          };
        };
      in
      {
        packages = {
          inherit bin;
          default = bin;
        } //
        (if system != "aarch64-darwin" then {
          # M1 and docker dont get along
          inherit dockerImage;
        } else { });

        devShells.default = pkgs.mkShell {
          # instead of passing `buildInputs` / `nativeBuildInputs`,
          # we refer to an existing derivation here
          inputsFrom = [ bin ];
        };
      }
    );
}
