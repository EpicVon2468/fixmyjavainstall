# Installation:

Latest version: `v0.4.0: "There is no v0.3 in ba sing se"`

You can [download the latest release](https://github.com/EpicVon2468/fixmyjavainstall/releases/latest/) if there is one for your system and architecture.

You can also install using cargo with:

```shell
cargo install --git https://github.com/EpicVon2468/fixmyjavainstall --tag v0.4.0
```

Or by running:

```shell
git clone https://github.com/EpicVon2468/fixmyjavainstall
cd fixmyjavainstall
cargo build --release --path .
```

If you would like to be able to install a JVM not made for your system, add `--features multi-os` to your installation command.

Additionally, please note that **there is no [crates.io](https://crates.io/) listing for fuji!**<br>
That is to say, `cargo install fuji` **WILL NOT INSTALL THIS PROJECT!**

## Documentation:

For standalone in-memory documentation:

```shell
fuji (subcommand(s)) --help
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