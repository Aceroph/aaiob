let
  pkgs = import (fetchTarball (
    "https://github.com/NixOS/nixpkgs/archive/3367e70de7e808f07a36632a0c2b1c7419671abc.tar.gz"
  )) { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    gtk4-layer-shell
    pkg-config
    pango
    cargo
    rustc
    gtk4
  ];
}
