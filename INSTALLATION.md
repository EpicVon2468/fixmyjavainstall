# Installation:

[![Release]][Release-1]

You can [download the latest release][Release-1] if there is one for your system and architecture.

You can also install using cargo with:

```shell
RUSTFLAGS="-C target-cpu=native -O" cargo install --git https://github.com/EpicVon2468/fixurjavainstall --tag v0.6.0
```

Or by running:

```shell
git clone https://github.com/EpicVon2468/fixurjavainstall
cd fixurjavainstall
RUSTFLAGS="-C target-cpu=native -O" cargo build --release --path .
```

If you would like to be able to install a JVM not made for your system, add `--features multi-os` to your installation command.

Additionally, please note that **there is no [crates.io](https://crates.io/) listing for fuji!** ~~(yet)~~<br>
That is to say, `cargo install fixurjavainstall` **WILL NOT INSTALL THIS PROJECT!** ~~(yet)~~

## Documentation:

For standalone in-memory documentation:

```shell
fuji --help
```

For UNIX `man` entries:

```shell
# if compiled with `--features dev` (installs to "$PWD/man", only works as long as "$PWD/man" is on the manpath)
fuji manual && export MANPATH="$PWD/man:$(manpath)"
# else (installs to "/usr/share/man")
sudo fuji manual
```

For rustdoc pages in a local website:

```shell
cargo clean --doc
RUSTDOCFLAGS="--default-theme=ayu" cargo doc --document-private-items --all-features --release --color=always --no-deps --open
```

[Release]: https://img.shields.io/github/v/release/EpicVon2468/fixurjavainstall?logo=github&label=latest%20version
[Release-1]: https://github.com/EpicVon2468/fixurjavainstall/releases/latest/