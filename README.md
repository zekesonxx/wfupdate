# wfupdate
[![Build Status](https://travis-ci.org/zekesonxx/wfupdate.svg?branch=master)](https://travis-ci.org/zekesonxx/wfupdate)

wfupdate is a small CLI utility to parse out the Warframe launcher's log file and spit out download progress. This is because, on Linux at least, the Warframe launcher won't show download progress under Wine.

I'd like to eventually fledge this out into a full Wine babysitter for Warframe.

This is the third version of this script, after [a bash version and a Node.js version](https://gist.github.com/zekesonxx/1a73236e7dff3b5bb847a7d1908bd252). The shell script version worked on line counts, not on byte counts, and was very inaccurate. The JavaScript version is essentially a less refined version of the initial commit of the Rust version.

## Use
Right now the input filename is hardcoded to "Preprocess.log" in the current directory, so you'll need to edit the source if you want to use it with an actual Warframe installation.

I've stuck a xz-compressed copy of the log I've been using to test [here](https://files.zekesonxx.com/Preprocess.log.xz) (2MB uncompressed), which should produce the output `bytes: 7 MB/4 GB 0.156%; files: 395/26212 1.506%`.
