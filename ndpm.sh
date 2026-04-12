#!/usr/bin/env bash
set -euo pipefail

PROGNAME=$(basename "$0")
NOID_XBPS_REPO="repository=https://github.com/noid-linux/xbps-repo/releases/latest/download"

usage() {
	cat <<-EOH
	Usage: $PROGNAME <command> [args...] [-- xbps options ...]

	COMMANDS
	 install <pkg...>	Install one or more packages
	 remove <pkg...>	Remove one or more packages
	 search <query>		Search for packages
	 update				Sync repository index
	 upgrade			Upgrade all packages
	 repo				Configure the noid XBPS repository

	OPTIONS
	 -h					Show this help and exit

	Wrapper script around xbps package manager.
	EOH
}

if [[ $EUID -eq 0 ]]; then
    echo "Error: Avoid running $PROGNAME as root/sudo" >&2
    exit 1
fi

while getopts "h" opt; do
case $opt in
    h) usage; exit 0;;
    *) usage >&2; exit 1;;
esac
done
shift $((OPTIND - 1))

if [[ $# -lt 1 ]]; then
    echo "Error: no command given" >&2
    usage >&2
    exit 1
fi
cmd="$1"
shift

case "$cmd" in
	install) sudo xbps-install -S "$@" ;;
	remove) sudo xbps-remove -R  "$@" ;;
	search)
		if [[ $# -lt 1 ]]; then
		    echo "Error: search requires a query" >&2
		    exit 1
		fi

		if [[ $# -gt 1 ]]; then
			echo "Warning: search takes one argument, ignoring extras" >&2
		fi
		xbps-query -Rs "$1"
		;;
	update) sudo xbps-install -S ;;
	upgrade)
		sudo sh -c 'xbps-install -Syu xbps && xbps-install -yu'
		;;
	repo)
		echo "$NOID_XBPS_REPO" | sudo tee \
			/etc/xbps.d/0-repository-noid.conf
		;;
	*)
		echo "Error: unknown command '$cmd'" >&2
		usage
		exit 1
		;;
esac
