{ config, pkgs, ... }:
{
  imports = [
    ./hardware-configuration.nix ./packet.nix
  ];

  nixpkgs.config = {
    allowUnfree = true;
  };

  boot.loader.grub.enable = true;
  boot.loader.grub.version = 2;

  time.timeZone = "America/Los_Angeles";

  environment.systemPackages = with pkgs; [
    wget
    alacritty
    vimHugeX
    bitcoin
    colordiff
    gitAndTools.gitFull
  ];

  users.extraUsers.rodarmor = {
    isNormalUser = true;
    uid = 1000;
  };

  system.stateVersion = "17.03";
}
