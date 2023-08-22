# SPDX-FileCopyrightText: 2023 The Booters Developers
#
# SPDX-License-Identifier: GPL-3.0-only

{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";

    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs, nixpkgs-mozilla }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;

          overlays = [
            (import nixpkgs-mozilla)
          ];
        };

        toolchain = (pkgs.rustChannelOf {
          date = "2023-08-19";
          channel = "nightly";
          sha256 = "sha256-SNA+Wwlw49SYWcfMF7S4QrJba7xonK9Z/SIZV8E4M9c=";
        }).rust.override {
            targets = [
                "x86_64-unknown-linux-musl"
                "thumbv7em-none-eabihf"
                "aarch64-unknown-linux-musl"
            ];
            extensions = [
                "rust-src"
                "rustfmt-preview"
                "llvm-tools-preview"
            ];
        };

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

      in
      rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ systemd.dev protobuf ];
        };

        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell {
          nativeBuildInputs = [ toolchain ] ++ (with pkgs; [ pkg-config ]);
          buildInputs = with pkgs; [ systemd.dev protobuf ];
        };
      }
    );
}
