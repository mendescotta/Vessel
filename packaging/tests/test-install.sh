#!/bin/sh
set -eu

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
install="$script_dir/install.sh"
launcher="$tmp/usr/libexec/vessel/run-build"
policy="$tmp/usr/share/polkit-1/actions/org.voidlinux.vessel.policy"
icon="$tmp/usr/share/icons/hicolor/scalable/apps/org.voidlinux.vessel.svg"

DESTDIR="$tmp" sh "$install" >/dev/null
[ -x "$launcher" ] || { echo "FAIL: launcher not installed executable"; exit 1; }
[ "$(stat -c %a "$launcher")" = 755 ] || { echo "FAIL: launcher mode $(stat -c %a "$launcher")"; exit 1; }
[ "$(stat -c %a "$policy")" = 644 ] || { echo "FAIL: policy missing or mode wrong"; exit 1; }
[ "$(stat -c %a "$icon")" = 644 ] || { echo "FAIL: icon missing or mode wrong"; exit 1; }
cmp -s "$launcher" "$script_dir/vessel-run-build" || { echo "FAIL: launcher differs from source"; exit 1; }
cmp -s "$icon" "$script_dir/../resources/images/org.voidlinux.vessel.svg" || { echo "FAIL: icon differs from source"; exit 1; }
echo "PASS: installs launcher, policy and icon"

DESTDIR="$tmp" sh "$install" --uninstall >/dev/null
[ ! -e "$launcher" ] && [ ! -e "$policy" ] && [ ! -e "$icon" ] && [ ! -d "$tmp/usr/libexec/vessel" ] || { echo "FAIL: uninstall left files"; exit 1; }
echo "PASS: uninstall removes them"

if DESTDIR="$tmp" sh "$install" --bogus >/dev/null 2>&1; then
    echo "FAIL: unknown option accepted"; exit 1
fi
echo "PASS: rejects unknown options"
