# wfupdate
[![Build Status](https://travis-ci.org/zekesonxx/wfupdate.svg?branch=master)](https://travis-ci.org/zekesonxx/wfupdate)

wfupdate is a small CLI utility to parse out the Warframe launcher's log file and spit out download progress. This is because, on Linux at least, the Warframe launcher won't show download progress under Wine.

I'd like to eventually fledge this out into a full Wine babysitter for Warframe.

This is the third version of this script, after [a bash version and a Node.js version](https://gist.github.com/zekesonxx/1a73236e7dff3b5bb847a7d1908bd252). The shell script version worked on line counts, not on byte counts, and was very inaccurate. The JavaScript version is essentially a less refined version of the initial commit of the Rust version.

I've stuck a xz-compressed copy of a log I've been using to test [here](https://files.zekesonxx.com/Preprocess.log.xz) (2MB uncompressed), which should produce the output `bytes: 7 MB/4 GB 0.156%; files: 395/26212 1.506%`.

# Progress
* Meta
  * [ ] Config File
* [x] Parse Warframe log outputs
  * [ ] Refine log parsing code to be smoother
* [ ] Launcherless:
  * [x] Game playing
  * [x] Updating
  * [ ] Repairing
  * [ ] "Optimizing"
* [ ] Launcher:
  * [ ] Running
  * [ ] Handling `Launcher.exe.tmp`
* [ ] GUI
  * [ ] Basic GUI
  * [ ] Setup Wizard
* [ ] Setup in clean wineprefix from scratch
  * [ ] Make a wineprefix
  * [ ] Run winetricks to install dependencies
  * [ ] Run `Warframe.msi` w/ user prompts
  * [ ] Use custom Wine versions (grab from PoL?)
  * [ ] PlayOnLinux integration
