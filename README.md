# Fix Ur Java Install – A JVM & Kotlin management utility.

(Re)writing this in Rust was simpler than debugging and 'fixing' the bash script(s).  I am serious.

NOTICE: You must run with `sudo -E` to preserve environment variables, else some preset detections may fail.<br>
DEVELOPER NOTICE: Expect regular breaking changes; Do not depend on `libfuji` as a stable API!

## Status:

- Linux: `[WORKING]-[TESTED]`
  - All core functionality is working, tested regularly.
- macOS: `[BROKEN*]-[TESTED]`
  - All core functionality _should_ be working, but isn't.
  - Rootless breaks many things.
  - `/usr/local/bin` is not on default path.
- Windows: `[WORKING?]-[UNTESTED]`
  - Symbolic links for `%JAVA_HOME%\bin\java.exe` & `%JAVA_HOME%\bin\javaw.exe` don't work and can't be fixed.
    - Feature has been disabled, so this should _not_ crash.  Support will be re-evaluated in future.
  - Despite being quoted, `"\Program Files\fuji\jvm\25"` isn't treated as one path, as batch fails to handle the space in `Program Files`.
    - No longer relevant because no batch scripts are used.  See above.