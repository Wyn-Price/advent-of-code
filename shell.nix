{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.cbc      # Cbc library
  ];

  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.cbc}/lib:$LD_LIBRARY_PATH
  '';
}