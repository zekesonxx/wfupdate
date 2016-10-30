# wfupdate
[![Build Status](https://travis-ci.org/zekesonxx/wfupdate.svg?branch=master)](https://travis-ci.org/zekesonxx/wfupdate)

wfupdate is a work-in-progress replacement to the official Warframe launcher. It started out as ([the third version of](https://gist.github.com/zekesonxx/1a73236e7dff3b5bb847a7d1908bd252)) a tool to parse the log file produced by the Warframe launcher, because the launcher is broken under Wine and simply says "Checking for new content..." indefinitely until it finishes updating. wfupdate now supports updating and launching the game under Wine, with preparations made to eventually support Windows as well.

The tool is mostly functional as-is, however it's missing numerous features, most notably a GUI and setting the game up in a fresh wineprefix.

----

The Warframe launcher works in two stages, Stage 1, which is done by the original launcher, and Stage 2, which is done by running Warframe.exe. You can find more information about it in `LAUNCHERPROTOCOL.md`. This tool can do both stages, although it still has a fair share of issues with both.

As for the log parsing part of it, I've stuck a xz-compressed copy of a log I've been using to test [here](https://files.zekesonxx.com/Preprocess.log.xz) (2MB uncompressed), which should produce the output `bytes: 7 MB/4 GB 0.156%; files: 395/26212 1.506%`.

# Limitations/Gotchas
* Can only deal with 32-bit Wine (despite the 64-bit Warframe executable switch and WINEARCH environment variables)
* Wineserver needs to be running before trying to update the game

##### *Past this point is sort of a mess. I use this README for planning and such, and this way it keeps it around in git.*


# Progress
* Meta
  * [x] Config File
  * [ ] Threading
* [x] Parse Warframe log outputs
  * [ ] Refine log parsing code to be smoother
* [ ] Launcherless:
  * [x] Game playing
  * [x] Updating
    * [x] Stage 1 Updating (replacing `Launcher.exe`)
      * [x] Checking which files need updates
      * [x] Updating those files
    * [x] Stage 2 Updating (running `Warframe.exe`)
      * [ ] Automatic restart if the download stalls
    * [ ] Git tracking of file changes
  * [ ] Repairing
  * [ ] "Optimizing", whatever the fuck that is
* [ ] ~~Launcher:~~
  * Not needed, thanks to REing the launcher protocol.
  * Should be able to operate 100% Launcherless
* [ ] GUI
  * [ ] Basic GUI
  * [ ] Setup Wizard
  * [ ] Editing game settings (video, chat, etc)
* [ ] Setup in clean wineprefix from scratch
  * [ ] Make a wineprefix
  * [ ] Run winetricks to install dependencies
  * [ ] Run `Warframe.msi` w/ user prompts
    * Uh, maybe.
    * Can setup all the necessary files without the msi
    * Does the game care about the registry?
    * Research needed
    * If it does care, can we manipulate the registry in our favor somehow?
    * And, if it does care, should we have a secondary Wine-side binary, or should we use Wine regedit to change things?
  * [ ] Use custom Wine versions (grab from PoL?)
  * [ ] PlayOnLinux integration
* [ ] Windows support
  * [ ] Disable Wine selection/managing in Windows
  * [ ] Run the game directly, without going through Wine
  * [ ] Deal with the lack of execvp(3)


# Semi-In-Order TODO List
* [x] Implement a config file
* [x] Rework the `paths` module
* [ ] Rework the `wine` module a `run` module (+ allow for future Windows support)
  * [x] dx10, dx11, language
  * [x] wineprefix setting
  * [ ] wine executable setting
  * [ ] 32bit/64bit switch
* [ ] Wine version selection
* [x] DX10/11 switches
* [ ] Rework the CLI with two-tiered subcommands, split into `cli` module
  * [x] Run
  * [x] Config
  * [ ] Update
  * [ ] Housekeeping (repair/"optimize")
* [ ] Game setup from scratch
* [ ] GUI

# CLI Release Checklist
* [ ] Wine executable/LD_LIBRARY_PATH setting
* [ ] Rework `update` into `cli` module
* [ ] Add an update check for stage 2 (kill the game at BytesToDownload)
* [ ] Rework `parse` into `cli` module
* [ ] Add repair command
* [ ] Fix up `wine-ver` into `wine`, get into `cli` module, implement setting to config vars
* [ ] `wfupdate wine winecfg` or something like that
* [ ] wineserver hosting (run in background as subcommand, stdout/err to /dev/null)
* [ ] Basic usage guide
