@echo off

set JAVA_HOME="/Program Files/fuji/jvm/25"

if defined CLASSPATH (
	echo foo
) else (
	echo bar
)

start /b "" "%JAVA_HOME%/bin/javaw.exe" %*