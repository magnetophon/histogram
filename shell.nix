let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in with nixpkgs;
  stdenv.mkDerivation {
    name = "moz_overlay_shell";
    nativeBuildInputs = [ pkg-config clang lld ];
    buildInputs = [
      (nixpkgs.rustChannelOf { date = "2024-04-20"; channel = "nightly"; }).rust
      rustup
      pkg-config
      openssl
      xorg.libxcb
      # xlibsWrapper

      glib
      atk
      cairo
      gdk-pixbuf
      pango
      gtk3

      udev
      alsaLib
      alsa-lib
      vulkan-loader
      libGL
      egl-wayland
      xorg.libXcursor
      xorg.libXrandr
      xorg.libXi
      libxkbcommon
    ];
    shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
       pkgs.lib.makeLibraryPath [ udev alsaLib vulkan-loader libGL libxkbcommon ]
     }"'';
  }
