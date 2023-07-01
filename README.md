# shadler
A shell script to stream and download anime from AllAnime

## Usage
`$ shadler anime` to watch anime and `$ shadler manga` to read manga.

## Dependencies
* curl
* sed
* grep
* mpv for video player

## Installation

### Termux
Using termux require you to have `mpv-android` installed on your phone. You can find it [here](https://github.com/mpv-android/mpv-android/releases)

```sh
curl -o shadler 'https://raw.githubusercontent.com/hithere-at/shadler/master/shadler'
cp shadler $PREFIX/bin
```

### GNU/Linux

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

Currently, there is no Windows support. It is possible to use PowerShell on Linux and do the development there but that is another mess i dont want to deal, dash (sh) is already a painful experience.

## To-do list
- [x] Core functionality (e.g streaming and downloding)
- [ ] `TODO` AllAnime API documentation
- [x] Support for other platforms
- [ ] Support for using arguments using `getopt`

## Notes
- Download option for manga is broken on Termux due to Android scoped storage. Opening the HTML file using `termux-open`  will result in `Failed to load image` error. This is due to Android 11+ scoped storage enforcement. Will add a fix until a workaround is found
