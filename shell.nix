let
  mozilla = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ mozilla ]; };
  rust = (nixpkgs.rustChannelOf {
    channel = "stable";
    date = "2020-04-23";
  }).rust.override { extensions = [ "rust-src" ]; };
in with nixpkgs; mkShell { buildInputs = [ rust openssl pkgconfig ]; }
