# Chip 8 emulator & debugger for rust
This is a Chip 8 emulator written in rust. It comes with a debugger that allows you to see the memory state of your chip 8 interpreter and all upcoming instructions.

All graphics and sound are made using the Sdl3 library.

![demo](assets/chip8-demo.gif "Demo")

## Requirements :
* Rust and Cargo are required to run this code. see : https://www.rust-lang.org/tools/install 
* Besides that, SDL3 is needed with sdl3 ttf visit https://wiki.libsdl.org/SDL3/ and https://wiki.libsdl.org/SDL3_ttf to find out how to install them.

## Usage :
simply run `cargo run` to execute the program.
The emulator uses the following key mapping between your keyboard and 
the gamepad
```
-----------     -----------
| 1 2 3 C |     | 1 2 3 4 |
| 4 5 6 D |  _  | A Z E R |
| 7 8 9 E |  _  | Q S D F |
| A 0 B F |     | W X C V |
-----------     -----------
```
(On the left is the gamepad and on the right is the corresponding keyboard keys)

Besides that, the four following special keys are used in the program : 
* F1: Starts and Pauses the emulation
* F2: Executes one single instruction in step-by-step mode
* F3: Restarts the emulation
* F4: Stops the emulation and exits the program

You can change a few values inside the code to change configuration.

* [main](src/main.rs) contains a variable `rom_path` you'll have to change in order to change the loaded and executed rom (sorry I did not make a file explorer 🙇‍♂️ )
* [screen_config](src/screen_config.rs) contains a lot of different constants to change the screen size, scale, and colors.
* [runner](src/runner.rs) contains an INTERPRETER_VARIANT constant you can change to select between the 2 available variants implemented : CosmacVip and Chip48



## Current development state
This project is some kind of research project I did for myself with myself to experiment with rust and familiarize myself with emulation.
Sadly, upon reaching the actual state of development of this code my motivation gradually fell and I don't really feel like picking this up to polish it and make it nice and clean. The fact that nobody may use it also diminished my motivation.

If you are using this project and would like me to keep working on it and make it cleaner, please say so that would be the extra push I need.

If you would like to contribute and help me clean this up, please refer to the next section.

## Contribute
First, if you have any review for me feel free to open an issue or open a pull request. This being my first ever Rust project, I know my code is far from perfect and would love to hear what you think about it.

As stated earlier, the code is not polished and there are still a lot of things to do on this project. If you would like to contribute feel free to open a pull request aswell. 

Here are some ideas I have about what can be improved, you can work on any of those, or anything else you'd like to improve, it's not restricted to these ideas : 
- [ ] Adding a configuration file for users to be able to change the (3) colors of the screen, keymapping, default rom folder, interpreter variant (CosmacVip or Chip48), etc...
- [ ] Adding a new special key (ex: F5) for Rom selection using a folder explorer (I would loooove for the explorer to be text based and to live inside the current SDL window, using my custom fond but that would be a lot of work)
- [ ] Making a build of this code that embarks Sdl3 so that people don't have to have it installed on their computers to use the emulator.
- [ ] Dealing more properly with errors : I know, unwrap is a brutal way to deal with errors that might crash very easylly. Especially a Sdl recognized sound driver is needed to execute the code while it shouldn't. But for now the emulator crashes if there is not one.

## Thanks and Inspiration
I would really like to thank [Tobias V. Langhoff](https://github.com/tobiasvl) for his blog post [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) that was my main inspiration and reference for this whole project.

I would also like to thank the developers of the test roms I used to validate my interpreter so thanks to 
* [corax89](https://github.com/corax89) for the [chip8-test-rom](https://github.com/corax89/chip8-test-rom)
* BonCodeur/BestCoder for the BC_Test (crazy, I've searched a long time everywhere and I can't find the repo anymore, prettysure I found it once...)