{
  description = "pakuvaraus";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      crane,
    }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ] (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        inherit (pkgs) lib;
        craneLib = (crane.mkLib pkgs).overrideToolchain (
          p:
          p.rust-bin.stable."1.98.0".default.override {
            extensions = [
              "rust-src"
              "rust-analyzer"
              "clippy"
              "rustfmt"
            ];
          }
        );

        src = lib.fileset.toSource {
          root = ./.;
          fileset = lib.fileset.unions [
            (craneLib.fileset.commonCargoSources ./.)
            ./migrations
            ./static
            ./templates
          ];
        };
        commonArgs = {
          inherit src;
          strictDeps = true;
          buildInputs = with pkgs; [
            openssl
            pkg-config
          ];
          env = {
            OPENSSL_DIR = "${pkgs.openssl.dev}";
            OPENSSL_LIB_DIR = "${pkgs.lib.getLib pkgs.openssl}/lib";
          };
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in
      {
        packages = {
          pakuvaraus = craneLib.buildPackage (
            commonArgs // {
              inherit cargoArtifacts;
              nativeBuildInputs = with pkgs; [
                sqlx-cli
                postgresql
              ];

              # Set up a temporary PostgreSQL database for sqlx
              preBuild = ''
                export PGDATA=$(mktemp -d)
                export PGHOST=$PGDATA

                initdb -U postgres
                pg_ctl start -o "-k $PGDATA -h \"\""

                createdb -U postgres pakuvaraus
                export DATABASE_URL="postgres://postgres@localhost/pakuvaraus?host=$PGDATA"
                cargo sqlx migrate run
              '';
            }
          );

          docker = pkgs.dockerTools.buildLayeredImage {
            name = "pakuvaraus";
            tag = "latest";

            contents = [
              self.packages.${system}.pakuvaraus
              pkgs.cacert
            ];

            config = {
              Cmd = [ "/bin/pakuvaraus" ];
              ExposedPorts."3000/tcp" = { };
            };
          };

          default = self.packages.${system}.pakuvaraus;
        };
        checks = {
          inherit (self.packages.${system}) pakuvaraus;

          formatting = pkgs.runCommand "cargo-fmt-check" {
            nativeBuildInputs = [ craneLib.cargo ];
          } ''
            cd ${src}
            cargo fmt --check
            touch $out
          '';
        };
        devShells.default = craneLib.devShell {
          checks = {
            inherit (self.packages.${system}) pakuvaraus;
          };

          inherit (commonArgs) env;
        };
      }
    );
}
