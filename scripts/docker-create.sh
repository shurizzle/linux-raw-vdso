#!/bin/sh

set -eux

SCRIPTPATH="$(
	cd -- "$(dirname "$0")" >/dev/null 2>&1
	pwd -P
)"

docker buildx build \
	-f "$SCRIPTPATH/../docker/test.dockerfile" \
	-t "linux-raw-vdso/test" \
	"$SCRIPTPATH/.."
