@echo off

setlocal enableextensions

if defined CLASSPATH (
	echo foo
) else (
	echo bar
)

start /b /wait "" "%JAVA_HOME%\bin\java.exe" %*