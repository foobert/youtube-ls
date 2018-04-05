# youtube-ls

[![Build Status](https://travis-ci.org/foobert/youtube-ls.svg?branch=master)](https://travis-ci.org/foobert/youtube-ls)

Parse the youtube website to find videos. Works great in tandem with [youtube-dl](https://rg3.github.io/youtube-dl/)!

So for example to download Critical Role via cron, you could use something along the lines of

    youtube-ls geekandsundry "Critical Role" | youtube-dl -f 22 -a - --download-archive archive.txt

Pre-compiled binaries for arm and armv7 are available from the [Github
releases](https://github.com/foobert/youtube-ls/releases) page.
