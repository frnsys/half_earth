/*
  This file...
  - Makes rustup, just, trunk & cargo-laptos available
  - Uses rustup to download Rust nightly & wasm32- target
  - Adds cargo to path
*/
{ pkgs ? import <nixpkgs> {} }:
let
  version = "nightly";
in
  pkgs.mkShell {
    nativeBuildInputs = with pkgs.buildPackages; [
      just
      rustup
      trunk
      cargo-leptos
    ];
    shellHook = ''
      rustup toolchain install ${version} --target wasm32-unknown-unknown
    '';
    RUSTUP_TOOLCHAIN = version;
    PATH="${builtins.getEnv "PATH"}:${builtins.getEnv "HOME"}/.cargo/bin";
}
