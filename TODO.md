# TODO:

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
- [ ] Automagic JVM selection
- [ ] `fuji manual install` ("think make vs. make install")
- [ ] custom sysroot/prefix
- [ ] Research / implement file locks
- [ ] Allow adding `Feature`s to already-installed JVMs (including non-fuji)
- [ ] `Append` for `features`