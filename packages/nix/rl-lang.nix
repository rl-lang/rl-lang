{ lib
, rustPlatform
, fetchFromGitHub
, pkg-config
, zlib
, stdenv
, darwin
}:

rustPlatform.buildRustPackage rec {
  pname = "rl-lang";
  version = "2.3.0";

  src = fetchFromGitHub {
    owner = "rl-lang";
    repo = "rl-lang";
    rev = "v${version}";
    hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="; # update with actual hash
  };

  cargoHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="; # update with actual hash

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [
    zlib
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];

  meta = with lib; {
    description = "Programming language with first-class VM and C transpiler";
    homepage = "https://github.com/rl-lang/rl-lang";
    license = with licenses; [ mit asl20 ];
    maintainers = [ ];
    mainProgram = "rl";
  };
}
