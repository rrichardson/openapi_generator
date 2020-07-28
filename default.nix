with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "openapi-generator-${version}";
  version = "0.1.1-alpha.0";
  src = ./.;
  buildInputs = [ ];

  checkPhase = "";
  cargoSha256 = "sha256:1xxq05bcplsbhxnmqjig26w0zz0xxfp3cc942jslqacf6h1sclss";

  meta = with stdenv.lib; {
    description = "Openapi generator";
    license = licenses.isc;
    maintainers = [ "easymov" ];
    platforms = platforms.all;
  };
}
