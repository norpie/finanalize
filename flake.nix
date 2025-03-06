{
  description = "Finanalyze development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              (rust-bin.stable.latest.default.override {
                extensions = ["rust-src" "rust-analyzer"];
              })
              openssl
              fontconfig
              pkg-config
              geckodriver
              # ra-multiplex
            ];

            shellHook = ''
              # systemd-run --user --same-dir --service-type=exec --setenv=PATH --setenv=CARGO_HOME "$SHELL" ra-multiplex server
            '';
          };
      }
    );
}
