# to be able to build plotters

let
  pkgs = import <nixpkgs> {};
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
        freetype fontconfig cmake
    ];
    nativeBuildInputs = with pkgs; [
        pkg-config
    ];
  }