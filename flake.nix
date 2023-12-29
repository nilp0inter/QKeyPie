{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
    };
  };

  description = "An open-source, Rust-based CLI tool for personalized key mapping and shortcut automation on Xencelabs Quick Keys";

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      flake = {
        homeManagerModules.default = ./home-module.nix;
      };
      imports = [
        inputs.flake-parts.flakeModules.easyOverlay
      ];

      systems = ["x86_64-linux" "aarch64-linux"];

      perSystem = {
        self',
        config,
        pkgs,
        ...
      }: {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            udev
            xdotool
          ];
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
          ];
        };

        overlayAttrs = {
          inherit (config.packages) qkeypie;
        };

        packages.qkeypie = pkgs.rustPlatform.buildRustPackage {
          pname = "qkeypie";
          version = "0.1.0";

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            udev
            xdotool
          ];

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          meta = with nixpkgs.lib; {
            description = "An open-source, Rust-based CLI tool for personalized key mapping and shortcut automation on Xencelabs Quick Keys";
            homepage = "https://github.com/nilp0inter/QKeyPie";
            license = licenses.gpl3;
            platforms = platforms.linux;
            maintainers = with maintainers; [nilp0inter];
            mainProgram = "qkeypie";
          };
        };

        packages.default = self'.packages.qkeypie;

        formatter = pkgs.alejandra;
      };
    };
}
