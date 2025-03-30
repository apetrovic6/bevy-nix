{ pkgs, lib, config, inputs, ... }:
let 

p = with pkgs; [
    pkg-config
    udev
    alsa-lib 
    alsa-plugins 
    alsa-tools
    alsa-utils
    vulkan-loader
    xorg.libX11 
    xorg.libXcursor 
    xorg.libXi 
    xorg.libXrandr # To use the x11 feature
    libxkbcommon 
    wayland
  ];
in {
  cachix.enable = false;
  # https://devenv.sh/packages/
  packages = p;

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = "nightly";
  };

  env = {
    LD_LIBRARY_PATH = lib.makeLibraryPath p;
    NIXPKGS_ALLOW_UNFREE = 1;
  };

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.run.exec = ''
    nix run --impure github:nix-community/nixGL -- cargo run
  '';

  enterShell = ''
    hello
    git --version
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # https://devenv.sh/git-hooks/
  # git-hooks.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
