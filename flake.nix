{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = inputs:
    let
      system = "x86_64-linux";
      pkgs = inputs.nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          vulkan-loader
          libGL
          alsa-lib
          xorg.libXi
          xorg.libX11
          xorg.libXrandr
          xorg.libXcursor
          xorg.libXinerama
        ];
        nativeBuildInputs = with pkgs; [
          cmake
          pkg-config
        ];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      };
    };
}
