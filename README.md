# shadler-rs
A rust console application to stream and download anime from AllAnime

## Insanity
We are announcing yet another `shadler` ported to a programming language, Rust!

### Why?
Rust memory safety ensures that the application cannot have memory leaks or undefined behaviour, making it safer and harder to break. We also got a new contributor to help code shadler from scratch.

## Usage
`$ shadler anime` to watch anime and `$ shadler manga` to read manga. `$ shadler help` to get more information.

## Dependencies
TODO

## Installation
TODO

### For Termux users
Using Termux requires you to have either [mpv-android](https://github.com/mpv-android/mpv-android), [NextPlayer](https://github.com/anilbeesetti/nextplayer), or [VLC](https://github.com/videolan/vlc-android) installed on your phone.

## Supported platform
- GNU/Linux
- Termux

> Windows port using WinUI 3 is on progress at [shadler-winui](https://github.com/hithere-at/shadler-winui), however it *will* only support Windows 10 and above. If someone wants to continue the PowerShell port, please do.

## To-do list
- [x] Core functionality (e.g streaming and downloding)
- [ ] `TODO` AllAnime API documentation
- [x] Support for other platforms
- [x] Support for using arguments

## Notes
- ~~Download option for manga is broken on Termux due to Android scoped storage. Opening the HTML file using `termux-open`  will result in `Failed to load image` error. This is due to Android 11+ scoped storage policy. Will add a fix until a workaround is found.~~ As a workaround, on Termux, reading offline will require Python 3 http.server module. This method does not use any data.
