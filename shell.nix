# save this as shell.nix
{pkgs ? import <nixpkgs> {}}: let
  rust-overlay =
    import (builtins.fetchTarball
      "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> {overlays = [rust-overlay];};
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      (pkgs.rust-bin.stable.latest.rust.override {extensions = ["rust-src"];})

      # keep this line if you use bash
      cargo
      clippy
      rust-analyzer
      rustfmt
      rustc
      just
      openapi-generator-cli
    ];
  }
