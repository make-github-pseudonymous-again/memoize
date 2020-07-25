mkdir a
mkdir b
mkdir c
mkdir a/d
mkdir a/e
mkdir a/d/z
function sep {
	echo '---'
	echo '---' >&2
}
memoize 1 ls
sep
cd a
memoize 1 ls
sep
ls
sep
sleep 2
memoize 1 ls
sep
memoize 1 ls d
sep
cd ..
memoize 1 ls d
sep
sleep 2
memoize 1 ls d
sep
