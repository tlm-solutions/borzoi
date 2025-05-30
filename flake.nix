{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";

    utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs = { self, nixpkgs, utils, ... }:
    utils.lib.eachSystem ["x86_64-linux" "aarch64-linux" ]
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};

          package = pkgs.callPackage ./package.nix { };

          makeTest = pkgs.callPackage "${nixpkgs}/nixos/tests/make-test-python.nix";

        in rec {
          checks.test-diesel-migration =
          let
            username = "postgres";
            password = "password";
            database = "database";
          in
          (makeTest
            {
              name = "test-diesel-migration";
              nodes = {
                server = { lib, config, pkgs, ... }: {
                  services.postgresql = {
                    enable = true;
                    ensureDatabases = [ database ];
                    ensureUsers = [{
                      name = username;
                    }];
                    initialScript = pkgs.writeScript "initScript" ''
                      ALTER USER postgres WITH PASSWORD '${password}';
                    '';
                  };

                  systemd.services.postgresql.postStart = lib.mkAfter ''
                    ${pkgs.diesel-cli}/bin/diesel migration run --database-url "postgres://${username}:${password}@localhost/${database}" --migration-dir ${self}/migrations
                    ${pkgs.diesel-cli}/bin/diesel migration redo --database-url "postgres://${username}:${password}@localhost/${database}" --migration-dir ${self}/migrations
                    ${pkgs.diesel-cli}/bin/diesel migration run --database-url "postgres://${username}:${password}@localhost/${database}" --migration-dir ${self}/migrations
                  '';
                };
              };
              testScript = ''
                start_all()
                server.wait_for_unit("postgresql.service")
                server.succeed("sudo -u postgres -- ${pkgs.diesel-cli}/bin/diesel print-schema --database-url postgres://${username}:${password}@localhost/${database} > schema.rs")
                server.copy_from_vm("schema.rs", "")
              '';
            }{
              inherit pkgs;
              inherit (pkgs) system;
            });
          packages = {
            borzoi = package;
            default = package;
            update-schema = pkgs.writeScriptBin "update-schema" ''
              #!/usr/bin/env bash
              
              nix build ${self}#checks.${system}.test-diesel-migration
              BUILD_DIR=$(nix build ${self}#checks.${system}.test-diesel-migration --no-link --print-out-paths)
              rm -rf src/schema.rs
              cp $BUILD_DIR/schema.rs src/schema.rs
            '';

            run-migration-borzoi = pkgs.writeScriptBin "run-migration" ''
              ${pkgs.diesel-cli}/bin/diesel migration run --migration-dir ${self}/migrations
            '';
          };
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = with packages.borzoi; buildInputs ++ nativeBuildInputs;
          };
        }
      ) // {
      overlays.default = final: prev: {
        inherit (self.packages.${prev.system})
          borzoi run-migration-borzoi;
      };

      nixosModules = rec {
        default = borzoi;
        borzoi = import ./nixos-module;
      };

      hydraJobs =
        let
          hydraSystems = [
            "x86_64-linux"
            "aarch64-linux"
          ];
        in
        builtins.foldl'
          (hydraJobs: system:
            builtins.foldl'
              (hydraJobs: pkgName:
                nixpkgs.lib.recursiveUpdate hydraJobs {
                  ${pkgName}.${system} = self.packages.${system}.${pkgName};
                }
              )
              hydraJobs
              (builtins.attrNames self.packages.${system})
          )
          { }
          hydraSystems;
    };
}
