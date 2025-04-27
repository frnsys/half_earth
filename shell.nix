/*
  This file...
  - Uses fenix to get a nightly rust toolbelt
  - Automatically installs runtime dependencies
  - Adds cargo to path
*/
{ pkgs ? import <nixpkgs> { 
  overlays = [ 
    (import "${fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz"}/overlay.nix")
  ]; 
}}:
  pkgs.mkShell {
    nativeBuildInputs = with pkgs.buildPackages; [
      just
      #cargo-leptos
      #trunk
      (with fenix; combine [
        latest.cargo
        latest.rustc
        targets.wasm32-unknown-unknown.latest.rust-std
      ])
    ];
    shellHook = ''
      cargo install trunk
      cargo install cargo-leptos
    '';
    PATH="${builtins.getEnv "PATH"}:${builtins.getEnv "HOME"}/.cargo/bin";
}