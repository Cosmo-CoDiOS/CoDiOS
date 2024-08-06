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

  outputs = { self, flake-utils, nixpkgs, nixpkgs-mozilla }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;

          overlays = [
            (import nixpkgs-mozilla)
          ];
        };

        toolchain = (pkgs.rustChannelOf {
          date = "2024-04-11";
          channel = "nightly";
          sha256 = "sha256-dPMfc+32T+p/DluUaN6qJk1+qAYbsYYMbZMmwzaldzs=";
        }).rust.override {
          targets = [
            "x86_64-unknown-linux-musl"
            "thumbv7em-none-eabihf"
            "aarch64-unknown-linux-musl"
          ];
          extensions = [
            "rust-src"
            "rustfmt-preview"
            "clippy-preview"
            "llvm-tools-preview"
            "rust-std"
          ];
        };
      in
      {
        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell {
          nativeBuildInputs = [ toolchain ] ++ (with pkgs; [ pkg-config ]);
          buildInputs = with pkgs; [ systemd.dev protobuf SDL SDL.dev SDL2 SDL2.dev ];
        };
      }
    );
}
