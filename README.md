# Fix Ur Java Install – A JVM & Kotlin management utility.

Rewriting this in Rust was simpler than debugging and 'fixing' the bash script(s).  I am serious.

TODO:

- Download Kotlin
- Download Kotlin Native

### Installation:

`cargo install --git https://github.com/EpicVon2468/fixmyjavainstall`

If you would like to be able to install a JVM not made for your system, add `--features multi_os` to your installation command.

### Documentation:

`cargo doc --no-deps --document-private-items --features multi_os --open`