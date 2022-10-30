# Theme
Set system-wide theme for linux rices  

| :warning: Project in early stage, things WILL change (and might break) |
| -- |

## Support
Currently supports [kitty terminal](https://sw.kovidgoyal.net/kitty/)

## Build and run
### Clone repository
```sh
git clone https://github.com/manankarnik/theme.git
```

### Change directory
```sh
cd theme
```

### Build project
```sh
cargo build
```

### Run project
```sh
cargo run -- set <Theme name>
```
| :warning: Theme based on `<Theme name>` must be defined in config.toml, or Default theme will be generated |
| -- |

## Configuration
The config file read is `$HOME/.config/theme/config.toml`

### Theme
Themes are defined as array of objects
| :warning: Theme will panic if any fields are missing |
| -- |

```toml
[[themes]]
name = "Dracula"
foreground = "#f8f8f2"
background = "#282a36"
color0 = "#21222c"
color1 = "#ff5555"
color2 = "#50fa7b"
color3 = "#f1fa8c"
color4 = "#bd93f9"
color5 = "#ff79c6"
color6 = "#8be9fd"
color7 = "#f8f8f2"
color8 = "#6272a4"
color9 = "#ff6e6e"
color10 = "#69ff94"
color11 = "#ffffa5"
color12 = "#d6acff"
color13 = "#ff92df"
color14 = "#a4ffff"
color15 = "#ffffff"


[[themes]]
name = "Monokai Dark"
foreground = "#f8f8f2"
background = "#272822"
color0 = "#272822"
color1 = "#f92672"
color2 = "#a6e22e"
color3 = "#f4bf75"
color4 = "#66d9ef"
color5 = "#ae81ff"
color6 = "#a1efe4"
color7 = "#f8f8f2"
color8 = "#75715e"
color9 = "#f92672"
color10 = "#a6e22e"
color11 = "#f4bf75"
color12 = "#66d9ef" 
color13 = "#ae81ff" 
color14 = "#a1efe4" 
color15 = "#f9f8f5" 


[[themes]]
name = "Solarized Dark"
foreground = "#93a1a1"
background = "#002b36"
color0 = "#002b36"
color1 = "#dc322f"
color2 = "#859900"
color3 = "#b58900"
color4 = "#268bd2"
color5 = "#6c71c4"
color6 = "#2aa198"
color7 = "#93a1a1"
color8 = "#657b83"
color9 = "#dc322f"
color10 = "#859900"
color11 = "#b58900"
color12 = "#268bd2" 
color13 = "#6c71c4" 
color14 = "#2aa198" 
color15 = "#fdf6e3"
```

### Kitty
Append the following to kitty.conf
```
include $HOME/.local/share/theme/kitty-colors.conf
```

| :exclamation: Auto reload will only work across all open windows if kitty is run with the [single\_instance](https://sw.kovidgoyal.net/kitty/invocation/#cmdoption-kitty-single-instance) option |
| -- |
