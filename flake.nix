{
  description = "Rust flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let 
      system = "aarch64-linux";
      pkgs = nixpkgs.legacyPackages."${system}";
    in {
      devShells."${system}".default = pkgs.mkShell {
        name = "rust-dev-shell";
        shellHook = ''

          rustup default stable

          echo "Welcome to the rust dev shell"
          nu
        '';

        packages = with pkgs; [
          cargo
          rustc
          rust-analyzer
          rustfmt
          clippy
        ];
      };
    };
}
