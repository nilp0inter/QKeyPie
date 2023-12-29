{ config, lib, pkgs, ... }:
let
  cfg = config.services.qkeypie;
  settingsFormat = pkgs.formats.toml { };
in
{
  options.services.qkeypie = {
    enable = lib.mkEnableOption "QKeyPie Daemon for the Xencelabs Quick Keys";

    package = lib.mkPackageOption pkgs "qkeypie" { };

    settings = lib.mkOption {
      type = settingsFormat.type;
      default = { };
      description = ''
        Settings for the QKeyPie Daemon for the Xencelabs Quick Keys

        - Example: https://github.com/nilp0inter/QKeyPie/blob/main/config.toml
        - Documentation: https://github.com/nilp0inter/QKeyPie/wiki/Configuration
      '';
    };
  };
  config = lib.mkIf cfg.enable {
    systemd.user.services."qkeypie" = {
      Unit = {
        Description = "QKeyPie Daemon for the Xencelabs Quick Keys";
        Documentation = "https://github.com/nilp0inter/QKeyPie";
      };
      Service = {
        ExecStart = "${lib.getExe cfg.package}";
        Restart = "always";
        RestartSec = 5;
      };
      Install.WantedBy = [ "default.target" ];
    };
    xdg.configFile."qkeypie/config.toml".source =
      if lib.isPath cfg.settings then
        cfg.settings
      else
        settingsFormat.generate "rio.toml" cfg.settings;
  };
}
