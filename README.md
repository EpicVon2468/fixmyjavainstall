# Fix Ur Java Install – A JVM & Kotlin management utility.

Rewriting this in Rust was simpler than debugging and 'fixing' the bash script(s).  I am serious.

TODO:

- Download Kotlin
- Download Kotlin Native

### Status:

- Linux: `[WORKING]-[TESTED]`
  - All core functionality working, tested regularly.
- macOS: `[WORKING]-[UNTESTED]`
  - All core functionality _theoretically_ working, needs testing.
- Windows: `[BROKEN]-[TESTED]`
  - Symbolic links for `%JAVA_HOME%\bin\java.exe` & `%JAVA_HOME%\bin\javaw.exe` don't work and can't be fixed.
  - Another dependency is needed for `.zip` file extraction.
  - No checks are done to make sure that `%PATH%` doesn't contain multiple of the same fuji install.

### Installation:

`cargo install --git https://github.com/EpicVon2468/fixmyjavainstall`

If you would like to be able to install a JVM not made for your system, add `--features multi_os` to your installation command.

### Documentation:

`cargo doc --no-deps --document-private-items --features multi_os --open`