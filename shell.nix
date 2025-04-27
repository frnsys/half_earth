/*
  This file...
  - Makes rustup & just vailable
  - Uses rustup to download a specific Nightly rust version
  - Automatically installs runtime dependencies
  - Adds cargo to path
*/
{ pkgs ? import <nixpkgs> { 
  overlays = [ 
    (import "${fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"}/default.nix")
  ]; 
}}:
let
  version = "nightly-2025-03-05";
in
  pkgs.mkShell {
    nativeBuildInputs = with pkgs.buildPackages; [
      just
      rustup
    ];
    shellHook = ''
      rustup toolchain install ${version} --target wasm32-unknown-unknown

      cargo install trunk
      cargo install cargo-leptos
    '';
    RUSTUP_TOOLCHAIN = version;
    PATH="${builtins.getEnv "PATH"}:${builtins.getEnv "HOME"}/.cargo/bin";
}