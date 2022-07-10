#!/bin/sh
# gitmoji as a commit hook
exec </dev/tty
gitmoji -v hook apply "$1" "$2"
