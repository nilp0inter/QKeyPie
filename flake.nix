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

  };
}
