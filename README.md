# vessel

Compose a Void Linux live ISO from a TOML profile. vessel generates a standalone `build.sh`
(no vessel or void-mklive needed to run it); the GTK4 app is an editor, with a profile also
usable from the command line.

```
vessel --generate profile.toml --out <dir>
cd <dir> && sudo ./build.sh        # ISO lands in out/<name>-<YYYYMMDD>.iso
```

## Profile

```toml
version = 1
name = "dinit-base"
arch = "x86_64"                  # x86_64 | x86_64-musl (musl: no multilib)
init = "dinit"                  # runit | dinit
userland = "gnu"
vanilla = false                 # runit: official Void console; dinit: keeps required boot packages
bootloaders = ["grub"]          # grub | limine | refind (refind: UEFI only); the GTK editor offers GRUB and rEFInd
desktops = []                   # gnome cosmic cinnamon xfce budgie kde; empty = console
display_manager = "none"        # none lightdm sddm gdm cosmic-greeter

[repos]
presets = ["voidlab"]           # voidlab nonfree multilib
custom = []

[packages]
base = []                       # replaces the init's base metapackage; empty = default
desktop = []                    # replaces the desktop presets' metapackages; empty = default
extra = []
exclude = []

[services]
enable = []
disable = []

[live]
loglevel = 4                    # kernel console level, 0–8; default 4
user = "anon"                   # live session user
# root_password = "..."        # optional; stored in the profile and build script
```

Optional `overlay_dir` (copied over the rootfs) and `post_rootfs_hook` (run in the rootfs chroot) are
relative to the profile. Examples are in `examples/`. The `voidlab` preset needs `VESSEL_VOIDLAB_REPO`: the
`repo/` directory of a local [voidlab-testing](https://github.com/mendescotta/voidlab-testing) checkout, or a repository URL.
There is no default, because the build installs with `-y`, which imports a remote repository's signing key
without asking: only point it at a URL whose key you trust. `WORK` and `OUT` override the script's scratch and
output folders.

`[packages].base` and `[packages].desktop` build a lean image from explicit package lists instead
of the `base-system`/desktop metapackages. A Dinit `base` list must include `dinit-void`, and `desktop`
needs at least one entry in `desktops`. Vanilla profiles cannot use either.

Vanilla profiles currently support console images only because desktop presets add optional
packages. Vanilla dinit profiles retain `base-system-dinit` and `dinit-void`
from the Voidlab repository. A custom root password is written to
`profile.toml` and `build.sh`; Vessel saves these files with owner-only permissions.

`[live].loglevel = 3` hides ordinary kernel errors from the boot console while
keeping them in the kernel log for `dmesg`. It changes console output only.

## Build

```
cargo build --release && cargo test
```

Targets `x86_64` and `x86_64-musl` (set `arch` in the profile; musl has no multilib). No disk images or package builds.

## References

- [void-mklive](https://github.com/void-linux/void-mklive): the Void live-ISO tooling the generated script follows; the `vmklive` dracut module in `assets/vmklive/` comes from it (license in `assets/vmklive/COPYING`).
- [noid-linux/noid-mklive](https://github.com/noid-linux/noid-mklive): the dinit-aware fork of void-mklive; the dinit changes in `assets/vmklive/` are adapted from it.
- [dinit-void](https://github.com/mendescotta/dinit-void): Void boot and shutdown services used by the `dinit` option.
- [Void Linux](https://voidlinux.org/) and [xbps](https://github.com/void-linux/xbps).

Licensed under GPL-3.0-or-later.
