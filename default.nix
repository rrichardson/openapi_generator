with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "openapi-generator-${version}";
  version = "0.1.1-alpha.0";
  src = ./.;
  buildInputs = [ ];

  checkPhase = "";
  cargoSha256 = "sha256:13iryn1k52phha7sm8la4dlmg2zbi4a1k22k66nksg1c3xi1zj6d";

  meta = with stdenv.lib; {
    description = "Openapi generator";
    license = licenses.isc;
    maintainers = [ "easymov" ];
    platforms = platforms.all;
  };
}
