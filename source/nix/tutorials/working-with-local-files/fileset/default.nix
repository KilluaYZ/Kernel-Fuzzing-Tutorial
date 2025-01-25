{
  system ? builtins.currentSystem,
  sources ? import ./npins,
}:
let
  pkgs = import sources.nixpkgs{
    config = { };
    overlays = [ ];
    inherit system;
  };
in
  pkgs.callPackage ./build.nix { }