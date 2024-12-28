# shadler
A shell script to stream and download anime from AllAnime

## Usage
`$ shadler anime` to watch anime and `$ shadler manga` to read manga. `$ shadler help` to get more information.

## Dependencies
* curl
* sed
* grep
* mpv for video player

## Installation

### Termux
Using Termux requires you to have either [mpv-android](https://github.com/mpv-android/mpv-android), [NextPlayer](https://github.com/anilbeesetti/nextplayer), or [VLC](https://github.com/videolan/vlc-android) installed on your phone.

```sh
curl -o shadler 'https://raw.githubusercontent.com/hithere-at/shadler/master/shadler'
cp shadler $PREFIX/bin
```

### GNU/Linux
VLC is supported but you have to install VLC using your package manager.

##### Ubuntu
```sh
curl -o shadler 'https://raw.githubusercontent.com/hithere-at/shadler/master/shadler'
sudo apt install -y mpv
sudo cp shadler /usr/local/bin
```

##### Arch linux
```sh
curl -o shadler 'https://raw.githubusercontent.com/hithere-at/shadler/master/shadler'
sudo pacman -S mpv
sudo cp shadler /usr/local/bin
```

## Supported platform
- GNU/Linux
- Termux

> I am currently trying to port this to PowerShell 4.0, which is very old and has been around since Windows 8.1. Please wait :D

## To-do list
- [x] Core functionality (e.g streaming and downloding)
- [ ] `TODO` AllAnime API documentation
- [x] Support for other platforms
- [x] Support for using arguments

### Windows (PowerShell) suppport
Currently, i am rewriting the entire code in PowerShell. However, the code might be similar because this is a direct port. This is a list of functions that has been rewritten in PowerShell:
- [x] int_sanitize -> Validate-Integer
- [x] int_prompt -> Prompt-Integer
- [x] get_query_url -> Get-QueryURL
- [ ] get_detail_url -> Get-DetailURL
- [ ] save_data -> Save-ShadlerData
- [ ] load_data -> Load-ShadlerData
- [ ] base_prompt -> Prompt-Base
- [ ] play_video -> Play-Video
- [ ] preparse_handler -> ???
- [x] show_help -> Show-ShadlerHelp
- [ ] anime_handler -> Handle-AnimeSubcommand
- [ ] manga_handler -> Handle-MangaSubcommand

## Notes
- ~~Download option for manga is broken on Termux due to Android scoped storage. Opening the HTML file using `termux-open`  will result in `Failed to load image` error. This is due to Android 11+ scoped storage policy. Will add a fix until a workaround is found.~~ As a workaround, on Termux, reading offline will require Python 3 http.server module. This method does not use any mobile data.
