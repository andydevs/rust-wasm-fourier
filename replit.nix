{ pkgs }: {
  deps = [
    pkgs.rustup
    pkgs.pkg-config
    pkgs.openssl
    pkgs.wasm-pack
    pkgs.cowsay
  ];
}