{ pkgs, config, lib, ... }:
let
  cfg = config.TLMS.borzoi;
in
{
  options.TLMS.borzoi = with lib; {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = ''Wether to enable borzoi service'';
    };
    http = {
      host = mkOption {
        type = types.str;
        default = "127.0.0.1";
        description = ''
          To which IP should bind.
        '';
      };
      port = mkOption {
        type = types.port;
        default = 8021;
        description = ''
          To which port should borzoi bind.
        '';
      };
    };

    database = {
      host = mkOption {
        type = types.str;
        default = "127.0.0.1";
        description = ''
          Database host
        '';
      };
      port = mkOption {
        type = types.port;
        default = 5354;
        description = ''
          Database port
        '';
      };
      user = mkOption {
        type = types.str;
        description = ''
          user for postgres
        '';
      };
      database = mkOption {
        type = types.str;
        description = ''
          postgres database to use
        '';
      };
      passwordFile = mkOption {
        type = types.oneOf [ null types.path types.string ];
        default = null;
        description = ''password file from which the postgres password can be read'';
      };
    };
    user = mkOption {
      type = types.str;
      default = "borzoi";
      description = ''systemd user'';
    };
    group = mkOption {
      type = types.str;
      default = "borzoi";
      description = ''group of systemd user'';
    };
    log_level = mkOption {
      type = types.str;
      default = "info";
      description = ''log level of the application'';
    };
    GRPC = mkOption {
      type = types.listOf
        (types.submodule {
          options.schema = mkOption {
            type = types.enum [ "http" "https" ];
            default = "http";
            description = ''
              schema to connect to GRPC
            '';
          };
          options.name = mkOption {
            type = types.str;
            default = "";
            description = ''
              GRPC name
            '';
          };
          options.host = mkOption {
            type = types.str;
            default = "127.0.0.1";
            description = ''
              GRPC: schema://hostname
            '';
          };
          options.port = mkOption {
            type = types.port;
            default = 50051;
            description = ''
              GRPC port
            '';
          };
        });
        default = [ ];
        description = ''list of grpc endpoint where borzoi should send data to'';
    };
  };

  config = lib.mkIf cfg.enable {

    systemd = {
      services = {
        "borzoi" = {
          enable = true;
          wantedBy = [ "multi-user.target" ];

          script = ''
            exec ${pkgs.borzoi}/bin/borzoi --host ${cfg.http.host} --port ${toString cfg.http.port}&
          '';

          environment = {
            "RUST_LOG" = "${cfg.log_level}";
            "RUST_BACKTRACE" = if (cfg.log_level == "info") then "0" else "1";
            "BORZOI_POSTGRES_PASSWORD_PATH" = "${cfg.database.passwordFile}";
            "BORZOI_POSTGRES_HOST" = "${cfg.database.host}";
            "BORZOI_POSTGRES_PORT" = "${toString cfg.database.port}";
            "BORZOI_POSTGRES_USER" = "${toString cfg.database.user}";
            "BORZOI_POSTGRES_DATABASE" = "${toString cfg.database.database}";
          } // (lib.foldl
            (x: y:
              lib.mergeAttrs x { "BORZOI_GRPC_HOST_${y.name}" = "${y.schema}://${y.host}:${toString y.port}"; })
            { }
            cfg.GRPC);

          serviceConfig = {
            Type = "forking";
            User = cfg.user;
            Restart = "always";
          };
        };
      };
    };

    # user accounts for systemd units
    users.users."${cfg.user}" = {
      name = "${cfg.user}";
      description = "This guy runs borzoi";
      isNormalUser = false;
      isSystemUser = true;
      group = cfg.group;
      uid = 1521;
    };
  };
}
