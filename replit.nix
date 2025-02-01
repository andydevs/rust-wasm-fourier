{ pkgs }: {
  deps = [
    pkgs.pkg-config
    pkgs.openssl
    pkgs.wasm-pack
    pkgs.rustup
    pkgs.cowsay
  ];
}