# Config Options
This is a list of config values wfupdate will care about.

Set them by running `wfupdate config-set <key> <value>`, ex `wfupdate config-set game:dx10 false`.

## game
* `dx10`: Enable DirectX 10 mode
* `dx11`: Enable DirectX 11 mode. Probably requires DirectX 10 mode to be enabled. I honestly don't know
* `language`: Two-character language code to pass to the game (`en`, `de`, etc)
* `64bit`: Run the game in 64-bit mode

# Planned (these don't function yet)
## game
* `mumble`: Load the Mumble overlay.

## update
* `steam`: Include Steam-specific assets when updating the game.
* `autorestart`: Takes a positive numerical value. Automatically restart the download if it hasn't downloaded a new file in the number of minutes specified.  
  I would recommend setting this to a reasonably high value relative to your Internet speed, in case it's downloading a big file. Setting this to 0 (or removing it) will disable automatic restarts.

## wine
* `wineprefix`: `WINEPREFIX` env var to use, for running the game and for finding paths. Defaults to the actual `WINEPREFIX` env var present when running wfupdate, and otherwise to `~/.wine` (use of default wineprefix not recommended).
* `winebin`: Path to the folder containing the Wine binaries to use to run the game. Defaults to `/usr/bin`.
* `winelib`: Path to the folder or folders (separated with `:`) containing libraries Wine needs, which will be prepended to `LD_LIBRARY_PATH`. Defaults to nothing.
