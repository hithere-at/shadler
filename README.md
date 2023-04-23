# shadler
A shell script to stream and download anime from AllAnime

## About this branch
Basically does the same thing but without Python. This shell script has better performance compared to the Python verison, but its kinda buggy at the moment. It has rewritten parser (using RegEx instead of accessing JSON). This script is also POSIX compatible. 

## Installation
Run the install script.
```sh
git clone https://github.com/hithere-at/shadler.git
cd shadler
chmod +x install && ./install
```

## Supported platform
For the time being, The only supported platform is Termux. Support for other platforms will be added until there is someone making a PR for it or until i get a PC. You still can use the the `adler` script though.

## To-do list
- [x] Core functionality (e.g streaming and downloding)
- [x] AllAnime API documentation 
- [ ] Support for other platforms
- [ ] Support for using arguments using `argparser` library
