{ lib, rustPlatform, pkg-config, cmake, openssl, libpq, ... }:

let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = manifest.name;
  inherit (manifest) version;

  src = lib.cleanSource ./.;

  cargoHash = "sha256-Z5EPAeiHV8aVJtKz7ENyTM2EYqY3gBVyUbHDw/y5tJg=";

  cargoBuildFlags = "-p ${finalAttrs.pname}";
  cargoTestFlags = "-p ${finalAttrs.pname}";

  nativeBuildInputs = [ pkg-config cmake ];

  buildInputs = [ openssl libpq ];

  meta = {
    mainProgram = "borzoi";
    description = "Simple rust server which collects sniffed data";
    homepage = "https://github.com/tlm-solutions/borzoi";
  };
})

