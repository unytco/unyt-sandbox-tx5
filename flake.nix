{
  description = "Flake for Holochain app development";

  inputs = {
    tauri-plugin-holochain.url = "github:darksoil-studio/tauri-plugin-holochain/main-0.6";
    holonix.url = "github:holochain/holonix?ref=main";

    nixpkgs.follows = "holonix/nixpkgs";
    flake-parts.follows = "holonix/flake-parts";
    playground.url = "github:darksoil-studio/holochain-playground?ref=main-0.5";
  };

  outputs = inputs@{ flake-parts, ... }: flake-parts.lib.mkFlake { inherit inputs; } {
    systems = builtins.attrNames inputs.holonix.devShells;
    perSystem = { inputs', pkgs, ... }: {
      formatter = pkgs.nixpkgs-fmt;

      devShells.default = pkgs.mkShell {
        inputsFrom = [
              inputs'.tauri-plugin-holochain.devShells.holochainTauriDev inputs'.holonix.devShells.default ];

        packages = (with pkgs; [
          nodejs_20
          binaryen
          inputs'.playground.packages.hc-playground
          yarn
          
        ]);

        shellHook = ''
          export PS1='\[\033[1;34m\][holonix:\w]\$\[\033[0m\] '
        '';
      };
      devShells.androidDev = pkgs.mkShell {
        inputsFrom = [
              inputs'.tauri-plugin-holochain.devShells.holochainTauriAndroidDev inputs'.holonix.devShells.default ];

        packages = (with pkgs; [
          nodejs_20
          binaryen
          inputs'.playground.packages.hc-playground
          yarn
          
        ]);

        shellHook = ''
          export PS1='\[\033[1;34m\][holonix:\w]\$\[\033[0m\] '
        '';
      };
    };
  };
}