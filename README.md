# Hyprland monitor attached

Run the user's script when you attach the monitor. When you detach your laptop from the monitor, and then attach it again, you no longer need to manually move your workspaces to the monitor if you make a simple script like in [example](https://github.com/coffebar/hyprland-monitor-attached/blob/main/added.sh) and setup it up with this program.

## How to use

Install this software and run with a path to the bash script as an argument.

It will listen to Hyprland's `monitoradded` and `monitorremoved` events and run your scripts.

The monitor's ID will be passed to the script as argument `$1`.

Your bash script may do any tasks you want, for example, move workspaces to the attached monitor, set wallpapers, or change bar configuration. See the example script [added.sh](https://github.com/coffebar/hyprland-monitor-attached/blob/main/added.sh) and [wiki](https://wiki.hyprland.org/Configuring/Dispatchers/).

## Install **hyprland-monitor-attached** from [AUR](https://aur.archlinux.org/packages/hyprland-monitor-attached)

```bash 
# e.g.
yay -Sy && yay -S hyprland-monitor-attached
```

and

Add this line to your hyprland.conf

```
exec-once = /usr/bin/hyprland-monitor-attached PATH_TO_ATTACHED_SHCRIPT.sh [PATH_TO_DETACHED_SHCRIPT.sh]
```

-----


## Install from cargo crates

```bash
cargo install hyprland-monitor-attached
```

Add this line to your hyprland.conf

```
exec-once = ~/.cargo/bin/hyprland-monitor-attached PATH_TO_ATTACHED_SHCRIPT.sh [PATH_TO_DETACHED_SHCRIPT.sh]
```


## Install from source

Install from source with **rustup**:

```bash

git clone https://github.com/coffebar/hyprland-monitor-attached.git
cd hyprland-monitor-attached

rustup override set stable
rustup update stable

cargo build --release

mkdir -p ~/.local/bin/
cp target/release/hyprland-monitor-attached ~/.local/bin/

```
Add this line to your hyprland.conf

```
exec-once = ~/.local/bin/hyprland-monitor-attached PATH_TO_ATTACHED_SHCRIPT.sh [PATH_TO_DETACHED_SHCRIPT.sh]
```

-----

## Contribution

Bug reports and PR are welcome. Thank you for your interest!

-----

Tested on Hyprland v0.30.
