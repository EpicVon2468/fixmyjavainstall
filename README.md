# Fix Ur Java Install – A JVM & Kotlin management utility.

Rewriting this in Rust was simpler than debugging and 'fixing' the bash script(s).  I am serious.

NOTICE: You must run with `sudo -E` to preserve environment variables, else some preset detections may fail.

## Status:

- Linux: `[WORKING]-[TESTED]`
  - All core functionality is working, tested regularly.
- macOS: `[BROKEN*]-[TESTED]`
  - All core functionality _should_ be working, but isn't.
  - Rootless breaks many things.
  - `/usr/local/bin` is not on default path.
- Windows: `[BROKEN]-[TESTED]`
  - Symbolic links for `%JAVA_HOME%\bin\java.exe` & `%JAVA_HOME%\bin\javaw.exe` don't work and can't be fixed.
  - Despite being quoted, `"\Program Files\fuji\jvm\25"` isn't treated as one path, as batch fails to handle the space in `Program Files`.

## Installation:

Latest version: `v0.1.5: "Windows, I think we should start seeing other people (I hate Windows pt. 4)"`

You can [download the latest release](https://github.com/EpicVon2468/fixmyjavainstall/releases/latest/) if there is one for your system and architecture.

You can also install using cargo with:

```shell
cargo install --git https://github.com/EpicVon2468/fixmyjavainstall
```

Or by running:

```shell
git clone https://github.com/EpicVon2468/fixmyjavainstall
cd fixmyjavainstall
cargo build --profile release --path .
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
RUSTDOCFLAGS="--default-theme=ayu" cargo doc --document-private-items --all-features --release --color=always --open
```

## TODO:

- [X] Download JVM
- [ ] Download Kotlin
- [ ] Download Kotlin Native
- [X] Look into why macOS `/usr/bin/*` is stubborn
  - [Oh dear...](https://apple.stackexchange.com/questions/193368/what-is-the-rootless-feature-in-el-capitan-really/)
  - Double oh dear: `/usr/local/bin` is not in `$PATH` by default.
- [ ] Get UNIX-likes to a stable release
- [ ] Fix Windows (soon™)
- [ ] Fix `cargo`-based installs not working with `sudo`
- [ ] Set `$JAVA_HOME` & `$PATH` persistently on UNIX-likes (or at least print export commands)
- [ ] Changelog?
- [ ] Automagic JDK selection
- [ ] `fuji manual install` ("think make vs. make install")
- [ ] custom sysroot/prefix