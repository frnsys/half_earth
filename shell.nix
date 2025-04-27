
{ pkgs ? import <nixpkgs> { 
  overlays = [ 
    (import "${fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz"}/overlay.nix")
  ]; 
}}:
  pkgs.mkShell {
    nativeBuildInputs = with pkgs.buildPackages; [
      just
      cargo-leptos
      trunk
      (fenix.complete.withComponents [
        "cargo"
      ])
    ];
}