{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    systems.url = "github:nix-systems/default";
    utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
  };

  outputs =
    {
      nixpkgs,
      utils,
      rust-overlay,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rust-bin = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "cargo"
            "rustc"
            "rust-analyzer"
            "clippy"
            "rustfmt"
            "rust-docs"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          RUST_SRC_PATH = "${rust-bin}/lib/rustlib/src/rust/library";

          buildInputs = [
            rust-bin

            # rust tooling
            pkgs.cargo-wizard # wizard for perf Cargo.toml
            pkgs.cargo-nextest
            pkgs.cargo-flamegraph
            pkgs.cargo-deny
            pkgs.cargo-edit
            pkgs.cargo-watch
            pkgs.mold

            # other tooling
            pkgs.reuse
            pkgs.hyperfine

            # the actual server package
            pkgs.riemann
            pkgs.openssl
          ];
        };
      }
    );
}
