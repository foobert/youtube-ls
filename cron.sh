#!/bin/bash
target/release/youtube-ls geekandsundry "Critical Role" | youtube-dl -a - --download-archive archive.txt
