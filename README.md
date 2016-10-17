# wfupdate
[![Build Status](https://travis-ci.org/zekesonxx/wfupdate.svg?branch=master)](https://travis-ci.org/zekesonxx/wfupdate)

wfupdate is a small CLI utility to parse out the Warframe launcher's log file and spit out download progress. This is because, on Linux at least, the Warframe launcher won't show download progress under Wine.

I'd like to eventually fledge this out into a full Wine babysitter for Warframe.

This is the third version of this script, after [a bash version and a Node.js version](https://gist.github.com/zekesonxx/1a73236e7dff3b5bb847a7d1908bd252). The shell script version worked on line counts, not on byte counts, and was very inaccurate. The JavaScript version is essentially a less refined version of the initial commit of the Rust version.

I've stuck a xz-compressed copy of a log I've been using to test [here](https://files.zekesonxx.com/Preprocess.log.xz) (2MB uncompressed), which should produce the output `bytes: 7 MB/4 GB 0.156%; files: 395/26212 1.506%`.

# Progress
* Meta
  * [ ] Config File
  * [ ] Threading
* [x] Parse Warframe log outputs
  * [ ] Refine log parsing code to be smoother
* [ ] Launcherless:
  * [x] Game playing
  * [ ] Updating
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
* [ ] Rework the `paths` module
* [ ] Merge the `wine` module into the reworked `paths` module (+ allow for future Windows support)
* [ ] Rework the CLI with two-tiered subcommands, split into `cli` module
* [ ] Wine version selection
* [ ] 32bit/64bit switch
* [ ] DX10/11 switches
