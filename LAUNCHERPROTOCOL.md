## The Launcher
* All info is based off of observations from `Launcher.log` and extracting `launcher.zip`.
* The launcher is a C++ application (`// call out to C++`) with a JavaScript frontend.
```js
// Send a query to the browser process.
function sendMessage(msg) {
    // Results in a call to the OnQuery method in binding_test.cpp
    window.cefQuery({
        request: 'Binding:' + msg,
        onSuccess: function (response) {
            //document.getElementById('result').value = 'Response: ' + response;
        },
        onFailure: function (error_code, error_message) { }
    });
}
```
There doesn't appear to be anything useful the JS code. Luckily, the update process very simple.

## Two Stage Update Process
The whole update process appears to be split in two, which I've numbered 1 and 2:
* Stage 1: Done by `Launcher.exe`, updates the game's executables and itself
* Stage 2: Done by `Warframe.exe`, run as a subprocess of `Launcher.exe`, updates the game's assets (textures, models, sounds, lua code, etc)

DX10 and DX11 assets are always downloaded, even if disabled. As well as all of the language-specific files, always downloaded.

64-bit assets however are not, they don't get downloaded if it's not enabled.


## Stage 1 Update Process
This is the one done by `Launcher.exe`.

Table of contents: `origin.warframe.com/origin/<hexvalue>/index.txt.lzma`  
`<hexvalue>` appears to just be a randomized 8 digit hex value, likely for cache busting.

lzmadec'd, the file looks like:

```
/Tools/Launcher.exe.F336FD22FDF21024C75FF46FE8F7A06E.lzma,313612
/Tools/Windows/x86/msvcr110.dll.4BA25D2CBE1587A841DCFB8C8C4A6EA6.lzma,351188
/Tools/Windows/x86/steam_api.dll.A83ADE32811F1419685E90F592ADF505.lzma,77655
/Tools/Windows/x86/symsrv.dll.64DEA54A4457371DEC27A4CFAE6EFB50.lzma,47951
/Warframe.exe.3BB594902B2E8037901ED9B2419E8FD5.lzma,6998205
```

The file paths all appear to correspond to files at `origin.warframe.com`, ex you can download [`origin.warframe.com/Tools/Launcher.exe.F336FD22FDF21024C75FF46FE8F7A06E.lzma`](http://origin.warframe.com/Tools/Launcher.exe.F336FD22FDF21024C75FF46FE8F7A06E.lzma).

The 32-digit hexadecimal number there is a MD5 hash of the extracted file.

The number after the comma is the size of the compressed file in bytes.

```
> /Tools/Launcher.exe.F336FD22FDF21024C75FF46FE8F7A06E.lzma,313612
$ wget content.warframe.com/Tools/Launcher.exe.F336FD22FDF21024C75FF46FE8F7A06E.lzma | grep Length
Length: 313612 (306K) [application/octet-stream]
```

#### List Contents
Interestingly, the list contains the giant game asset files too:
```
/Cache.Windows/F.TextureDx9.cache.09F145B086119F595F1D32047FCB9A2B.lzma,8541175817
```
Yes, it's 8145.5MB. However, `Launcher.exe` doesn't appear to ever touch these files. Only `Warframe.exe` touches them. Probably because they're massive and can be incrementally downloaded and updated by `Warframe.exe`, but (apparently) not by the launcher.


## Stage 2 Update Process
This is the one done by `Warframe.exe`.

Honestly, I can't see any value in doing this manually.

The `.toc` and `.cache` files are a custom format. You can find a forum post with a tool [here][toolfourm], ([mega mirror][megamirror] of the tool because the site is broken, courtesy of [this reddit user][redditthx]). However, there's no real harm in running `Warframe.exe` to update. WF.exe might stall all the time, but it *is* the reference implementation of how to deal with these files. And it's not a standard file format.

  [toolfourm]: http://forum.xentax.com/viewtopic.php?f=32&t=10782
  [megamirror]: https://mega.nz/#!EBk2xL7S!cgkfOof_7-EmB9CqdPGlKow4CcXJ2gW8oNsK7C1Y_yw
  [redditthx]: https://www.reddit.com/r/Warframe/comments/2ifbd6/ripping_the_music_from_the_game/cl2xuv0
