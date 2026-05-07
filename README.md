# Fix Ur Java Install – A JVM & Kotlin management utility.

[![Release]][Release-1]
[![Repository]][Repository-1]
[![Licence]][Licence-1]
[![Dependencies]][Dependencies-1]
[![Built With Ratatui][Ratatui]][Ratatui-1]

(Re)writing this in Rust was simpler than debugging and 'fixing' the bash script(s).  I am serious.

DEVELOPER NOTICE: Expect regular breaking changes; Do not depend on `libfuji` as a stable API!

MSRV: N/A (see: `rust-toolchain.toml`).

## Status:

- Linux: `[WORKING]-[TESTED]`
  - All core functionality is working, tested regularly.
- macOS: `[WORKING?]-[UNTESTED]`
  - All core functionality _should_ be working.
  - Rootless breaks many things.
- Windows: `[WORKING?]-[UNTESTED]`
  - Symbolic links for `%JAVA_HOME%\bin\java.exe` & `%JAVA_HOME%\bin\javaw.exe` don't work and can't be fixed.
    - Feature has been disabled, so this should _not_ crash.  Support will be re-evaluated in future.
  - Despite being quoted, `"\Program Files\fuji\jvm\25"` isn't treated as one path, as batch fails to handle the space in `Program Files`.
    - No longer relevant because no batch scripts are used.  See above.

## Installation:

See [INSTALLATION.md](https://github.com/EpicVon2468/fixurjavainstall/blob/master/INSTALLATION.md#installation)

[Repository]: https://img.shields.io/badge/git-EpicVon2468/fixurjavainstall-blue?logo=github
[Repository-1]: https://github.com/EpicVon2468/fixurjavainstall/

[Licence]: https://img.shields.io/badge/licence-MIT%20OR%20Apache--2.0-blue
[Licence-1]: https://github.com/EpicVon2468/fixurjavainstall/blob/master/LICENCE/

[Dependencies]: https://img.shields.io/deps-rs/repo/github/EpicVon2468/fixurjavainstall
[Dependencies-1]: https://deps.rs/repo/github/EpicVon2468/fixurjavainstall/

[Release]: https://img.shields.io/github/v/release/EpicVon2468/fixurjavainstall?logo=github
[Release-1]: https://github.com/EpicVon2468/fixurjavainstall/releases/latest/

[Ratatui]: https://img.shields.io/badge/Built_With_Ratatui-000?logo=ratatui&logoColor=fff
[Ratatui-1]: https://ratatui.rs/