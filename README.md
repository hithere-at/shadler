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

~~Currently, there is no Windows support. It is possible to use PowerShell on Linux and do the development there but that is another mess i dont want to deal, dash (sh) is already a painful experience.~~

~~Windows supports *may* come until i get a new keyboard. The current one has broken key which makes typing annoying~~

Nah, i cant. Too much pain

## To-do list
- [x] Core functionality (e.g streaming and downloding)
- [ ] `TODO` AllAnime API documentation
- [x] Support for other platforms
- [x] Support for using arguments

## Notes
- ~~Download option for manga is broken on Termux due to Android scoped storage. Opening the HTML file using `termux-open`  will result in `Failed to load image` error. This is due to Android 11+ scoped storage policy. Will add a fix until a workaround is found.~~ As a workaround, on Termux, reading offline will require Python 3 http.server module. This method does not use any data.
