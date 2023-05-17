{ naersk
, src
, lib
, pkg-config
, cmake
, protobuf
, postgresql_14
, zlib
, openssl
}:

naersk.buildPackage {
  pname = "borzoi";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = lib.fakeSha256;

  nativeBuildInputs = [ pkg-config cmake ];
  buildInputs = [ protobuf zlib postgresql_14 openssl ];

  meta = {
    description = "Simple rust server which collects data from tetra stations";
    homepage = "https://github.com/tlm-solutions/borzoi";
  };
}
