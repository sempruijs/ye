# ye

A command line interface for getting random quotes from [Kanye West](https://en.wikipedia.org/wiki/Kanye_West).
Written in Rust, with the kanye.rest api. 

## installing

### nix

add the following input to your flake:

```nix
  ye.url = "github:sempruijs/ye";
```

and then adding it to home.packages

```nix
    home.packages = with pkgs; [
        inputs.ye.packages.${pkgs.system}.default
    ]
```