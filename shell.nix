let
  mozilla = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ mozilla ]; };
  rust = (nixpkgs.rustChannelOf {
    channel = "1.48.0";
    sha256 = "sha256:0b56h3gh577wv143ayp46fv832rlk8yrvm7zw1dfiivifsn7wfzg";
  }).rust.override { extensions = [ "rust-src" ]; };
in with nixpkgs; mkShell { buildInputs = [ rust openssl pkgconfig ]; }
