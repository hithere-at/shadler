# shadler
A shell script to stream and download anime from AllAnime

## Usage
`$ shadler anime` to watch anime. `$ shadler music` to listen to anime songs

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

Support for Windows is not likely to come as i dont have access to Windows PC.

## To-do list
- [x] Core functionality (e.g streaming and downloding)
- [x] AllAnime API documentation 
- [ ] Support for other platforms
- [ ] Support for using arguments using `getopt`
