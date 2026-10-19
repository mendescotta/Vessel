#!/bin/sh
# -*- mode: shell-script; indent-tabs-mode: nil; sh-basic-offset: 4; -*-
# ex: ts=8 sw=4 sts=4 et filetype=sh

# The image ships without a machine-id (vessel strips the build root's), so
# give every live boot its own; dbus and /etc read the same value.
if [ ! -s "${NEWROOT}/etc/machine-id" ]; then
    sed 's/-//g' /proc/sys/kernel/random/uuid > "${NEWROOT}/etc/machine-id"
    chmod 0444 "${NEWROOT}/etc/machine-id"
fi
mkdir -p "${NEWROOT}/var/lib/dbus"
rm -f "${NEWROOT}/var/lib/dbus/machine-id"
ln -s /etc/machine-id "${NEWROOT}/var/lib/dbus/machine-id"
