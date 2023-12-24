{ pkgs ? import <nixpkgs> { overlays = [ (import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz)) ]; } }:

with pkgs;

mkShell {
  nativeBuildInputs = [
    python311
    python311Packages.z3
    python311Packages.setuptools
  ];
}