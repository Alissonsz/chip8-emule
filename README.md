# Chip8 emulator

A Chip8 emulator written in [rust](https://www.rust-lang.org/), using [OpenGL](https://www.opengl.org/) for graphics rendering and [GLFW](https://www.glfw.org/) for creating a OpenGL context and handle user input.

## Modules
##### CPU
* Contains the emulated interpreter itself, containing the CPU, Display and Keypad representations. The CPU module is responsible for running the programs, decoding and executing each instruction.

##### Renderer
* The emulator renderer, wrote using OpenGL. This renderer works receiving a 2D texture and drawing it on the window.

##### Web Renderer
* This one is just a small idea about maybe code a web renderer using webassembly, but nothing was implemented yet.

## How it works
The main file, located at src/main is responsible for create the emulator instance, giving the chip8 program code. It's also who runs the main loop, calling the functions that proccess the user inputs and telling the emulator how many cycles to run in each iteration. In the end of each iteration, the main loop get the emulator display state represented by an 1byte per pixel texture, that texture is given to the renderer module and rendered on the window.