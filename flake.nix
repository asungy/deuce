{
  description = "I don't know what this is yet.";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
    nixvim.url       = "github:nix-community/nixvim";
  };

  outputs = { self, nixpkgs, flake-utils, nixvim, }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        name = "deuce";
        pkgs = import nixpkgs { inherit system; };
        nvim = nixvim.legacyPackages.${system}.makeNixvim {
          extraPlugins = with pkgs.vimPlugins; [
            lazy-nvim
          ];
          extraConfigLua = ''
            require("lazy").setup({
              dir = ".",
            })
          '';
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            nvim
          ];
          shellHook = ''
            PS1="\n\[\033[01;32m\]${name}(default) >\[\033[00m\] "
          '';
        };
      }
    );
}
