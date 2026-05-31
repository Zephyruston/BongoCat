#!/bin/bash
# 强制 X11 backend — 宿主机 XDG_SESSION_TYPE=wayland 会泄漏进沙箱，
# 导致 WebKitGTK 去尝试 Wayland 然后连接失败。
export GDK_BACKEND=x11
# 尝试不禁用 WebKit 沙箱 — 宿主机上沙箱开启时应用正常工作
exec /app/bin/bongo-cat "$@"
