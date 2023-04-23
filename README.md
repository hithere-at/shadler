# shadler
A shell script to stream and download anime from AllAnime

## Usage
`$ shadler anime` to watch anime. `$ shadler music` to listen to anime songs

## Dependencies
* curl
* sed
* grep
* mpv for video player
* ffmpeg for m3u8 downloader

## Installation
Run the install script.
```sh
git clone https://github.com/hithere-at/shadler.git
cd shadler
chmod +x install && ./install
```

## Supported platform
For the time being, The only supported platform is Termux. Support for other platforms will be added until there is someone making a PR for it or until i get a PC. You still can use the the `shadler` script though.

## To-do list
- [x] Core functionality (e.g streaming and downloding)
- [x] AllAnime API documentation 
- [ ] Support for other platforms
- [ ] Support for using arguments using `getopt`
