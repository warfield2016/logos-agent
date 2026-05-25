{
  description = "Logos Autonomous AI Agent Module (LP-0008)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    # Official Logos module builder — calls logos-cpp-generator to emit the Qt
    # plugin shim from agent-core's C header. We never write C++ by hand.
    logos-module-builder.url = "github:logos-co/logos-module-builder";

    # Rust toolchain pinned per rust-toolchain.toml
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, logos-module-builder, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in {
        # `nix develop` — Rust toolchain + Logos build deps + Risc0
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            pkg-config
            openssl
            cmake
            just
            git
          ];
          shellHook = ''
            echo "Logos Agent dev shell ready."
            echo "  cargo build --workspace    # Rust crates"
            echo "  nix build .#module          # full module (.so + Qt shim)"
            echo "  just test                   # integration tests"
          '';
        };

        # `nix build .#module` — full Logos Core module via official builder.
        # Auto-generates Qt plugin shim from agent-core's cbindgen header.
        packages.module = logos-module-builder.lib.mkLogosModule {
          src = ./.;
          configFile = ./metadata.json;
          flakeInputs = inputs;
        };

        # Default build
        packages.default = self.packages.${system}.module;
      });
}
