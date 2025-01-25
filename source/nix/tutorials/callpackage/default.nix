let 
  pkgs = import <nixpkgs> {};
in
rec {
  # callPackage会自动填充参数
  hello = pkgs.callPackage ./hello.nix { audience = "people"; };
  hello-folks = hello.override { audience = "folks"; };
}