let pkgs = import <nixpkgs> {};
# choose the ocaml version you want to use
ocamlPackages = pkgs.ocaml-ng.ocamlPackages_4_11;
my-python = pkgs.python39;
python-with-packages = my-python.withPackages ( p: with p; [
    aiohttp
]);
in
pkgs.mkShell {
    # build tools
    nativeBuildInputs = with ocamlPackages; [ ocaml ]; #ocaml-lsp ];
    # dependencies
    buildInputs = with pkgs; [ 
        ocaml_make
        cargo
        rustc
        python-with-packages
        pkg-config
        openssl
    ];
}
