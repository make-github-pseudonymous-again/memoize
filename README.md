# memoize

Cache executable calls.

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
