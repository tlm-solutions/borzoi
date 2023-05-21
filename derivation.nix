{ buildPackage, src, lib, pkg-config, cmake, protobuf, postgresql, zlib, openssl }:

buildPackage {
  pname = "borzoi";
  version = "0.1.0";

  src = ./.;

  cargoSha256 = lib.fakeSha256;

  nativeBuildInputs = [ pkg-config cmake ];
  buildInputs = [ protobuf zlib postgresql openssl ];

  meta = with lib; {
    description = "Simple rust server which collects sniffed data";
    homepage = "https://github.com/tlm-solutions/borzoi";
  };
}
