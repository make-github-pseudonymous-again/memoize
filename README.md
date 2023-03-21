:floppy_disk: memoize
[![License](https://img.shields.io/github/license/make-github-pseudonymous-again/memoize.svg?style=flat)](https://raw.githubusercontent.com/make-github-pseudonymous-again/memoize/main/LICENSE)
[![Tests](https://img.shields.io/github/actions/workflow/status/make-github-pseudonymous-again/memoize/ci:test.yml?branch=main&event=push&label=tests)](https://github.com/make-github-pseudonymous-again/memoize/actions/workflows/ci:test.yml?query=branch:main)
[![Code coverage](https://img.shields.io/codecov/c/github/make-github-pseudonymous-again/memoize.svg)](https://codecov.io/gh/make-github-pseudonymous-again/memoize)
==

Cache executable calls.

## :minidisc: Install [![AUR package](https://img.shields.io/aur/version/memoize)](https://aur.archlinux.org/packages/memoize)

```sh
make DESTDIR=/ PREFIX=/usr install
```

## :woman_astronaut: Usage

```shell
$ cat fib
n="$1"
if [ "$n" -eq 0 ] ; then echo 0
elif [ "$n" -eq 1 ] ; then echo 1
else
	a="$(MEMOIZE_CACHE="/tmp/${USER}-memoize" memoize 9999 sh fib $((n-1)))"
	b="$(MEMOIZE_CACHE="/tmp/${USER}-memoize" memoize 9999 sh fib $((n-2)))"
	echo "$a + $b" | bc
fi
$ sh fib 100
354224848179261915075
$ sh fib 200
280571172992510140037611932413038677189525
```
