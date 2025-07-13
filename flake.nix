{
  description = "A Nix-flake-based Rust development environment";

 inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-25.05";
  };

  outputs = inputs:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forEachSupportedSystem = f: inputs.nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.self.overlays.default
          ];
        };
      });
    in
    {
      overlays.default = final: prev: {
        rustToolchain = with inputs.fenix.packages.${prev.stdenv.hostPlatform.system};
          combine (with stable; [
            clippy
            rustc
            cargo
            rustfmt
            rust-src
          ]);
      };

      devShells = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.mkShell rec {
          packages = with pkgs; [
            rustToolchain
            pkg-config
            cargo-deny
            cargo-edit
            cargo-watch
            rust-analyzer
						wasm-tools
            clang-tools            
            llvmPackages_latest.clang.cc
            lld
            just
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libxcb
            libxkbcommon
            vulkan-loader
            wayland
          ];
          shellHook = ''
              export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath packages)}";
            '';
          env = {
            # Required by rust-analyzer
            RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
          };
        };
      });
    };
}
