#!/usr/bin/env bash
set -euo pipefail

app_id="${1:-com.ayangweb.BongoCat}"

echo "Checking Flatpak input-device permission for ${app_id}"
flatpak info --show-permissions "${app_id}"

echo
echo "Checking evdev opens inside the sandbox"
flatpak run --command=sh "${app_id}" -c '
found=0
failed=0

for device in /dev/input/event*; do
  [ -e "${device}" ] || continue
  found=1
  if exec 3< "${device}"; then
    printf "readable: %s\n" "${device}"
    exec 3<&-
  else
    printf "denied:   %s\n" "${device}" >&2
    failed=1
  fi
done

if [ "${found}" -eq 0 ]; then
  echo "No /dev/input/event* devices are exposed inside the sandbox" >&2
  exit 1
fi

if [ "${failed}" -ne 0 ]; then
  echo "At least one evdev device cannot be opened" >&2
  exit 1
fi
'
