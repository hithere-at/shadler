# shadler
A PowerShell script to stream and download anime

## Usage
`> shadler anime` to watch anime and `$ shadler manga` to read manga. `> shadler help` to get more information.

## Dependencies
* curl
* sed
* grep
* mpv for video player

## Installation

TODO

## Supported platform
- Windows

> I am currently trying to port this to PowerShell 4.0, which is very old and has been around since Windows 8.1. Please wait :D

### Windows (PowerShell) suppport
Currently, i am rewriting the entire code in PowerShell. However, the code might be similar because this is a direct port. This is a list of functions that has been rewritten in PowerShell:

- [x] int_sanitize -> Validate-Integer
- [x] int_prompt -> Prompt-Integer
- [x] get_query_url -> Get-QueryURL
- [x] get_detail_url -> Get-DetailURL
- [x] get_streams_url -> Get-StreamURL
- [ ] save_data -> Save-ShadlerData
- [ ] load_data -> Load-ShadlerData
- [x] base_prompt -> Shadler-BasePrompt
- [ ] play_video -> Play-Video
- [x] preparse_handler -> **reworked**
- [x] show_help -> Show-ShadlerHelp
- [ ] anime_handler -> Handle-AnimeSubcommand
- [ ] manga_handler -> Handle-MangaSubcommand

I also plan to tidy up the code instead of porting it 1:1 with the original. For now, range input validation (Validate-Integer, Validate-InputRange) is reworked for better readability.

## Notes
- This might get scrapped in the future in favor of [shadler-avalon](https://github.com/hithere-at/shadler-avalon)
