:floppy_disk: memoize
[![License](https://img.shields.io/github/license/aureooms/memoize.svg?style=flat)](https://raw.githubusercontent.com/aureooms/memoize/main/LICENSE)
[![Build](https://img.shields.io/travis/aureooms/memoize/main.svg)](https://travis-ci.org/aureooms/memoize/branches)
[![Code coverage](https://img.shields.io/codecov/c/github/aureooms/memoize.svg)](https://codecov.io/gh/aureooms/memoize)
==

Cache executable calls.

## :minidisc: Install [![AUR package](https://img.shields.io/aur/version/memoize)](https://aur.archlinux.org/packages/memoize)

```sh
make DESTDIR=/ PREFIX=/usr install
```

## :woman_astronaut: Usage

```shell
$ memoize 60 ls
a
b
c
$ cd a
$ memoize 60 ls
a
b
c
$ ls
d
e
$ sleep 60
$ memoize 60 ls
d
e
$ memoize 60 ls d
z
$ cd ..
$ memoize 60 ls d
z
$ sleep 60
$ memoize 60 ls d
ls: cannot access 'd': No such file or directory
```
