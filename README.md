# Fix Ur Java Install – A JVM & Kotlin management utility.

Rewriting this in Rust was simpler than debugging and 'fixing' the bash script(s).  I am serious.

NOTICE: You must run with `sudo -E` to preserve environment variables, else some preset detections may fail.

### Status:

- Linux: `[WORKING]-[TESTED]`
  - All core functionality working, tested regularly.
- macOS: `[BROKEN]-[TESTED]`
  - All core functionality _should_ work, but doesn't.
  - Symbolic link install phase is broken.  See TODO.
- Windows: `[BROKEN]-[TESTED]`
  - Symbolic links for `%JAVA_HOME%\bin\java.exe` & `%JAVA_HOME%\bin\javaw.exe` don't work and can't be fixed.
  - Another dependency is needed for `.zip` file extraction.
  - No checks are done to make sure that `%PATH%` doesn't contain multiple of the same fuji install.
  - Despite being quoted, `"\Program Files\fuji\jvm\25"` isn't treated as one path, as batch fails to handle the space in `Program Files`.

### Installation:

`cargo install --git https://github.com/EpicVon2468/fixmyjavainstall`

If you would like to be able to install a JVM not made for your system, add `--features multi_os` to your installation command.

### Documentation:

`cargo doc --no-deps --document-private-items --features multi_os --open`

### TODO:

- [x] Download JVM
- [ ] Download Kotlin
- [ ] Download Kotlin Native
- [ ] Look into why macOS `/usr/bin/*` is stubborn