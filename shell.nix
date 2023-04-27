let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  # choose the ocaml version you want to use
  ocamlPackages = nixpkgs.ocaml-ng.ocamlPackages_4_11;
  my-python = nixpkgs.python310;
  python-with-packages = my-python.withPackages ( p: with p; [
    aiohttp
  ]);
in
  nixpkgs.mkShell {
    # build tools
    nativeBuildInputs = with ocamlPackages; [ ocaml ]; #ocaml-lsp ];
    # dependencies
    buildInputs = with nixpkgs; [ 
        #nixpkgs.latest.rustChannels.nightly.rust
	(nixpkgs.rustChannelOf { date = "2023-04-26"; channel = "nightly"; }).rust
	ocaml_make
        cargo
        rustc
        python-with-packages
        pkg-config
        openssl
    ];
  }
