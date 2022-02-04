let
  sources = import ./nix/sources.nix;
  rustPlatform = import ./nix/rust.nix { inherit sources; };
  pkgs = import sources.nixpkgs { overlays = [ (import sources.rust-overlay) ]; };
in
  with pkgs;

  pkgs.mkShell {
    buildInputs = [
      rustPlatform.rust.cargo
      openssl
      pkg-config

      git
      wasmtime
      wasm-pack
      cargo-watch
    ];
  }
