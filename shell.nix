let pkgs = import <nixpkgs> {};
# choose the ocaml version you want to use
ocamlPackages = pkgs.ocaml-ng.ocamlPackages_4_11;
in
pkgs.mkShell {
    # build tools
    nativeBuildInputs = with ocamlPackages; [ ocaml ]; #ocaml-lsp ];
    # dependencies
    buildInputs = with pkgs; [ ocaml_make ] ;
}
