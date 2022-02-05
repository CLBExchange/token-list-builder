{
  description = "Certified Token List.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    saber-overlay.url = "github:saber-hq/saber-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, saber-overlay, flake-utils }:
    flake-utils.lib.eachSystem [
      "aarch64-darwin"
      "x86_64-linux"
      "x86_64-darwin"
    ] (system:
      let
        pkgs = import nixpkgs { inherit system; }
          // saber-overlay.packages.${system};
        ci = pkgs.buildEnv {
          name = "ci";
          paths = with pkgs;
            [ cargo-workspaces libiconv ] ++ (lib.optionals stdenv.isDarwin
              (with pkgs.darwin.apple_sdk.frameworks; [ openssl Security ]));
        };
      in {
        packages.ci = ci;
        devShell =
          pkgs.mkShell { buildInputs = with pkgs; [ ci cargo-readme ]; };
      });
}
