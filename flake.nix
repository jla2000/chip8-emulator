{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };
      craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default.override {
        extensions = [ "llvm-tools-preview" ];
      });
      commonArgs = {
        src = pkgs.lib.sourceFilesBySuffices ./. [ ".toml" ".lock" ".rs" ".wgsl" ];
        buildInputs = with pkgs; [
          vulkan-loader
          alsa-lib
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          xorg.libXinerama
        ];
        nativeBuildInputs = with pkgs; [
          pkg-config
          cmake
        ];
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      main-crate = craneLib.buildPackage (commonArgs // {
        inherit cargoArtifacts;
      });
    in
    {
      checks.${system} = {
        inherit main-crate;

        clippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
        });

        fmt = craneLib.cargoFmt (commonArgs // {
          inherit cargoArtifacts;
        });

        test = craneLib.cargoTest (commonArgs // {
          inherit cargoArtifacts;
        });
      };

      packages.${system} = {
        default = main-crate;

        doc = craneLib.cargoDoc (commonArgs // {
          inherit cargoArtifacts;
        });

        coverage = craneLib.cargoLlvmCov (commonArgs // {
          inherit cargoArtifacts;
          cargoLlvmCovExtraArgs = "--html --output-dir $out";
        });
      };

      devShells.${system}.default = craneLib.devShell {
        inputsFrom = [ main-crate ];
      };
    };
}
