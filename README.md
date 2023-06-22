# shadler
A shell script to stream and download anime from AllAnime

## Usage
`$ shadler anime` to watch anime, `$ shadler music` to listen to anime songs, and `$ shadler manga` to read manga.

## Dependencies
* curl
* sed
* grep
* mpv for video player
* ffmpeg for m3u8 playlist downloader

## Installation

### Termux
Using termux require you to have `mpv-android` installed on your phone. You can find it [here](https://github.com/mpv-android/mpv-android/releases)

```sh
curl -o shadler 'https://raw.githubusercontent.com/hithere-at/shadler/master/shadler'
apt install -y ffmpeg
mv shadler $PREFIX/bin
```

### GNU/Linux

##### Ubuntu
```sh
curl -o shadler 'https://raw.githubusercontent.com/hithere-at/shadler/master/shadler'
sudo apt install -y ffmpeg mpv
sudo mv shadler /usr/local/bin
```

##### Arch linux
```sh
curl -o shadler 'https://raw.githubusercontent.com/hithere-at/shadler/master/shadler'
sudo pacman -S ffmpeg mpv
sudo mv shadler /usr/local/bin
```

## Supported platform
- GNU/Linux
- Termux

Currently, there is no Windows support. It is possible to use PowerShell on Linux and do the development there but that is another mess i dont want to deal, dash (sh) is already a painful experience.

## To-do list
- [x] Core functionality (e.g streaming and downloding)
- [ ] AllAnime API documentation `TODO`
- [x] Support for other platforms
- [ ] Support for using arguments using `getopt`
