# Wasite
Wasite is the WASI Terminal Environment API standard.  It defines a set of
environment variables and stdin/stdout parsing rules for interfacing with a
generic terminal.  It is **not** developed by the same people as WASI, and may
conflict with WASI's future goals (future WASI features may deprecate Wasite
features).  Wasite's only purpose is to extend WASI to make it possible to write
interactive terminal applications using WASI today.

Wasite aims to stay as close as possible to what you might expect to find on a
standard POSIX-like terminal for environment variables and parsing of stdin and
stdout.  Wasite uses a subset of the ANSI escape sequences.

This document is the official specification for Wasite.
