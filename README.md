# chip8-emulator ![build status](https://github.com/jla2000/chip8-emulator/actions/workflows/rust.yml/badge.svg)


My own take on a very simple chip8 emulator, using rust and the wgpu project.


## Features

- Cross-platform
- Fast wgpu backend
- Working Keyboard & Audio


## Usage

Download any chip8 rom from the internet.
Eg. _space_invaders.ch8_:

```sh
cd chip8-emulator
cargo run -- space_invaders.ch8
```


## TODO's

-  Implement multithreading to reduce lag.


## Screenshots

![Screenshot 1](./screenshots/1.png)
![Screenshot 2](./screenshots/2.png)
