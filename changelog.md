# Changelog

## v1.0
Initial release. Only has anime subcommand

## v1.1
Use `ffmpeg` for downloadig HLS playlist. I think this update also have shell rewrite, though bisecting will be a painful process

## v2.0
Addition of `manga` subcommand

## v2.1
- Fix API URL change. `api.allanime.to/allanimeapi` -> `api.allanime.day/api` and set `Referer`  header to fix website down error message
- Delete music subcommand as they *seems* to not provide anime musics anymore

## v2.2
- Added options support
- Added streaming using Next Player. Available only on Android
- Added streaming using VLC media player. Available on both platforms

## v2.3
- Fix anime download
- Removingg Yt-SD related codes as they have dropped the source

## v2.4
- Fix manga not appearing because of special characters, specifically double quotes e.g `"Oshi no Ko"`