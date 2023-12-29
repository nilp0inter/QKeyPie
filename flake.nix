{
  description = "An open-source, Rust-based CLI tool for personalized key mapping and shortcut automation on Xencelabs Quick Keys";

  outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in {

    devShells.x86_64-linux.default = pkgs.mkShell {
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

    packages.x86_64-linux.qkeypie = pkgs.rustPlatform.buildRustPackage {
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
        maintainers = with maintainers; [ nilp0inter ];
      };
    };
    packages.x86_64-linux.default = self.packages.x86_64-linux.qkeypie;
  };
}
