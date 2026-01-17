# shadler-rs
A rust console application to stream and download anime from AllAnime

### Whats the difference between the rust and shell version
Nothing. The rust version is a little bit faster than the shell version because of the advantage of using a compiled language. Other than that, it is pretty much the same and reaches feature parity with the shell version

## Usage
`$ shadler anime` to watch anime and `$ shadler manga` to read manga. `$ shadler help` to get more information.

## Dependencies
- rust, to compile rust programs (you can remove it when you are done)
- mpv for video player
  

## Installation
You can use the binary on the release page and execute them normally, or you can use the command below to do it for you automatically.
```sh
git clone https://github.com/hithere-at/shadler
cd shadler
cargo build --release
sudo mv target/release/shadler /usr/local/bin
```

### For Termux users
Using Termux requires you to have either [mpv-android](https://github.com/mpv-android/mpv-android), [NextPlayer](https://github.com/anilbeesetti/nextplayer), or [VLC](https://github.com/videolan/vlc-android) installed on your phone. Installation is a little bit different as `/usr/local/bin` doesn't exist. You can run the commands below to install it for you.
```sh
git clone https://github.com/hithere-at/shadler
cd shadler
cargo build --release
sudo mv target/release/shadler $PREFIX/bin
```

## Supported platform
- Linux (Ubuntu, Arch, Fedora, etc.)
- Termux

> Windows port using WinUI 3 is on progress at [shadler-winui](https://github.com/hithere-at/shadler-winui), however it *will* only support Windows 10 and above. If someone wants to continue the PowerShell port, please do.

## To-do list
- [x] Core functionality (e.g streaming and downloding)
- [ ] `TODO` AllAnime API documentation
- [x] Support for other platforms
- [x] Support for using arguments

## Notes
- Offline reading is possible on Termux, however due to the nature of Android, web browsers do not have direct access to folders and file. Therefore, to circumvent this problem, we used Python HTTP server to serve images. This does not require any online connection as the image is stored locally and its entire purpose is to load the image from `shadler` data folder.
