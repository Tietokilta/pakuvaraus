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

        # TypeScript client generated from the API's OpenAPI spec, see client/README.md
        inherit (pkgs) nodejs;
        pnpm = pkgs.pnpm_11;
        clientPnpmDeps = pkgs.fetchPnpmDeps {
          pname = "pakuvaraus-client";
          src = lib.fileset.toSource {
            root = ./client;
            fileset = lib.fileset.unions [
              ./client/package.json
              ./client/pnpm-lock.yaml
            ];
          };
          inherit pnpm;
          fetcherVersion = 4;
          hash = "sha256-Ex6bVxt+bBUM0keBo6UYlXluguQS55kwL9LpUC6kzo8=";
        };
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

          # OpenAPI description of the public API, generated from the handlers
          openapi-spec = pkgs.runCommand "openapi.json" { } ''
            ${self.packages.${system}.pakuvaraus}/bin/pakuvaraus-openapi > $out
          '';

          # The publishable npm package: types generated from openapi-spec,
          # typechecked, and packed with the files listed in package.json
          api-client = pkgs.stdenvNoCC.mkDerivation {
            pname = "pakuvaraus-client";
            inherit ((lib.importJSON ./client/package.json)) version;
            src = lib.fileset.toSource {
              root = ./client;
              fileset = lib.fileset.difference ./client (lib.fileset.maybeMissing ./client/node_modules);
            };
            pnpmDeps = clientPnpmDeps;
            nativeBuildInputs = [
              nodejs
              pnpm
              pkgs.pnpmConfigHook
            ];
            buildPhase = ''
              cp ${self.packages.${system}.openapi-spec} openapi.json
              pnpm run generate
              pnpm run typecheck
            '';
            installPhase = ''
              export HOME=$TMPDIR
              mkdir -p $out
              tar xzf "$(npm pack --silent)" --strip-components=1 -C $out
            '';
          };

          default = self.packages.${system}.pakuvaraus;
        };
        checks = {
          inherit (self.packages.${system}) pakuvaraus;
          inherit (self.packages.${system}) api-client;
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
          packages = [
            nodejs
            pnpm
          ];
        };
      }
    );
}
