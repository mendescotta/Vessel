#!/bin/sh
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root="${DESTDIR:-}"
launcher="$root/usr/libexec/vessel/run-build"
policy="$root/usr/share/polkit-1/actions/org.voidlinux.vessel.policy"
icon="$root/usr/share/icons/hicolor/scalable/apps/org.voidlinux.vessel.svg"

case "${1:-}" in
"")
	install -Dm755 "$here/vessel-run-build" "$launcher"
	install -Dm644 "$here/org.voidlinux.vessel.policy" "$policy"
	install -Dm644 "$here/../resources/images/org.voidlinux.vessel.svg" "$icon"
	echo "Installed $launcher"
	echo "Installed $policy"
	echo "Installed $icon"
	;;
--uninstall)
	rm -f "$launcher" "$policy" "$icon"
	rmdir "$root/usr/libexec/vessel" 2>/dev/null || true
	echo "Removed $launcher, $policy and $icon"
	;;
*)
	echo "usage: [sudo] $0 [--uninstall]" >&2
	exit 2
	;;
esac
