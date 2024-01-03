{
  description = "I don't know what this is yet.";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        name = "deuce";
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            jupyter
          ];
          shellHook = ''
            PS1="\n\[\033[01;32m\]${name}(default) >\[\033[00m\] "
          '';
        };
      }
    );
}
