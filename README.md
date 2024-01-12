# youtube-ls

This project has been archived.

My personal workflow evolved and I recommend either listening to the podcast or using yt-dlp with the following settings instead:

```
yt-dlp \
  -o "/path/to/output/%(title)s.%(ext)s" \
  -f 18 \
  --download-archive /var/local/critical-role/archive/archive \
  --match-title 'Talks Machina|Critical Role|Exandria Unlimited' \
  --playlist-reverse \
  --playlist-end 20 \
  'https://www.youtube.com/c/criticalrole/videos'
```
