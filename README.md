# PngMe
Along my journey to learn Rust, I found mention of a project to work at https://picklenerd.github.io/pngme_book/introduction.html.

As a newcomer to Rust, here is my implementation of the project

## Functionality

The commandline tool can add Chunks to a PNG file containing hidden messages.

It can then be used to recover the message from within the PNG file, or remove it, or print it.

## How To Use
``` pngme encode ./image.png ruSt "secret message here" ./output.png ```

``` pngme decode ./image.png ruSt "secret message here" ```

``` pngme remove ./image.png ruSt ```

``` pngme print ./image.png ```
