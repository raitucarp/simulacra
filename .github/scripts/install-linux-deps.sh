#!/usr/bin/env bash
# System libraries gpui (X11, Wayland, fonts) and the native file dialog (GTK 3) link against.
set -euo pipefail
sudo apt-get update
sudo apt-get install -y --no-install-recommends \
  pkg-config clang \
  libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libx11-xcb-dev libxcb1-dev \
  libvulkan-dev libfontconfig1-dev libfreetype-dev libgtk-3-dev libasound2-dev \
  libssl-dev libzstd-dev
