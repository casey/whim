{ pkgs ? import <nixpkgs> {} }:
with pkgs;
stdenv.mkDerivation {
  name = "whim";
  buildInputs = [
    openssl
  ];
  shellHook = ''
    export OPENSSL_DIR="${openssl.dev}"
    export OPENSSL_LIB_DIR="${openssl.out}/lib"
    export RUST_BACKTRACE=0
  '';
}
