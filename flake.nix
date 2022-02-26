{
    inputs = {
        flake-utils.url = "github:numtide/flake-utils";
        naersk.url = "github:nix-community/naersk";
    };

    outputs = { self, nixpkgs, flake-utils, naersk }:
        flake-utils.lib.eachDefaultSystem (
            system: let
                pkgs = nixpkgs.legacyPackages."${system}";
                naersk-lib = naersk.lib."${system}";
                buildInputs = with pkgs; [ 
                    pkg-config 
                    alsa-lib 
                    libudev
                    xorg.libXcomposite
                    xorg.libXtst
                    xorg.libXrandr
                    xorg.libXext
                    xorg.libX11
                    xorg.libXfixes
                    xorg.xkeyboardconfig
                    xorg.libpciaccess
                    xorg.libxcb
                    xorg.libXdamage
                    xorg.libxshmfence
                    xorg.libXxf86vm
                    xorg.libXinerama
                    xorg.libXdamage
                    xorg.libXcursor
                    xorg.libXrender
                    xorg.libXScrnSaver
                    xorg.libXxf86vm
                    xorg.libXi
                    xorg.libSM
                    xorg.libICE
                    xorg.libXt
                    xorg.libXmu
                    xorg.libxcb
                    xorg.libXft
                    libGL
                    libva
                    pipewire.lib
                    libGLU
                    mesa.drivers
                    mesa.llvmPackages.llvm.lib
                    vulkan-loader
                    freeglut
                    libvdpau
                    clang
                    lld
                ];
            in
                rec {
                    # `nix build`
                    packages.beeline = naersk-lib.buildPackage {
                        inherit buildInputs;
                        pname = "beeline";
                        root = ./.;
                    };
                    defaultPackage = packages.beeline;

                    # `nix run`
                    apps.beeline = flake-utils.lib.mkApp {
                        drv = packages.beeline;
                    };
                    defaultApp = apps.beeline;

                    devShell = pkgs.mkShell {
                        nativeBuildInputs = with pkgs; [ rustc cargo rust-analyzer rustfmt ] ++ buildInputs;
                        shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath (with pkgs; [
                            alsa-lib
                            udev
                            vulkan-loader
                        ])}"'';
                    };
                }
        );
}
