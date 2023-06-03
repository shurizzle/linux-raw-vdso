#!/bin/sh

set -eux

SCRIPTPATH="$(
	cd -- "$(dirname "$0")" >/dev/null 2>&1 || true
	pwd -P
)"

if ! (docker images --format '{{.Repository}}' | grep "^linux-raw-vdso/test\$"); then
	"$SCRIPTPATH/docker-create.sh"
fi

exec docker run --rm -it \
	--user "$(id -u):$(id -g)" \
	-v "$(realpath "${SCRIPTPATH}/.."):/project" \
	-v "$HOME:$HOME" \
	-e "HOME=$HOME" \
	"linux-raw-vdso/test" /bin/bash
