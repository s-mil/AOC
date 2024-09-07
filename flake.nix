{
  description = "Adevent of Code devshell for rs, c#, python, & go";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
  let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
  in
  {
    # importing package example
    # packages."x86_64-linux".default = 
    #   pkgs.callPackage (import ./default.nix) {};

    devShells."x86_64-linux".default = pkgs.mkShell {

      packages = [ 
        pkgs.go 
        pkgs.python312 
        pkgs.black
        pkgs.dotnetCorePackages.sdk_8_0 ];

      inputsFrom = [ pkgs.bat ];
      
    };
  };
}
