# Fix Ur Java Install – A JVM & Kotlin management utility.

Rewriting this in Rust was simpler than debugging and 'fixing' the bash script(s).  I am serious.

NOTICE: You must run with `sudo -E` to preserve environment variables, else some preset detections may fail.

## Status:

- Linux: `[WORKING?]-[UNTESTED]`
  - All core functionality _should_ be working.
- macOS: `[BROKEN*]-[TESTED]`
  - All core functionality _should_ be working, but isn't.
  - `/usr/local/bin` is not on default path.
- Windows: `[BROKEN]-[TESTED]`
  - Symbolic links for `%JAVA_HOME%\bin\java.exe` & `%JAVA_HOME%\bin\javaw.exe` don't work and can't be fixed.
  - Another dependency is needed for `.zip` file extraction.
  - Despite being quoted, `"\Program Files\fuji\jvm\25"` isn't treated as one path, as batch fails to handle the space in `Program Files`.

## Installation:

Latest version: `v0.1.2: "I hate macOS JVM .tar.gz files (and Windows (.zip) pt. 2)"`

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

If you would like to be able to install a JVM not made for your system, add `--features multi_os` to your installation command.

Additionally, please note that there is no [crates.io](https://crates.io/) listing for fuji!<br>
That is to say, `cargo install fuji` **WILL NOT INSTALL THIS PROJECT!**

## Documentation:

For rustdoc pages in a local website:

```shell
cargo doc --no-deps --document-private-items --all-features --open
```

For local UNIX `man` entries:

If the `dev` feature is enabled (installs to `$PWD/man`):

```shell
fuji manual && export MANPATH="$(manpath):$PWD/man"
```

Else (installs to `/usr/share/man`):

```shell
sudo fuji manual
```

For standalone in-memory documentation:

```shell
fuji <subcommand(s)> --help | less
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
- [ ] Set `$JAVA_HOME` & `$PATH`