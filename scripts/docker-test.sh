#!/bin/sh

set -eux

SCRIPTPATH="$(
	cd -- "$(dirname "$0")" >/dev/null 2>&1 || true
	pwd -P
)"

if ! (docker images --format '{{.Repository}}' | grep "^linux-raw-vdso/test\$"); then
	"$SCRIPTPATH/docker-create.sh"
fi

tty=
if [ -t 0 ]; then
	tty=-it
fi

exec docker run --rm $tty \
	--user "$(id -u):$(id -g)" \
	-v "$(realpath "${SCRIPTPATH}/.."):/project" \
	-v "$HOME:$HOME" \
	-e "HOME=$HOME" \
	"linux-raw-vdso/test" ./scripts/test.sh
