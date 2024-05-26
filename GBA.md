# GBA

A 2.9 inch screen with a 240x160 pixel resolution.
15-bit RGB color.
8-bit raw audio

> The Gba singleton struct is a crucial part of agb game development. It is used for almost all interactions with the Game Boy Advance's hardware, such as graphics rendering, timer access and audio playback.

## Buttons

The GBA has 10 buttons

- **D-pad**: Arrow Keys
- **A**: X
- **B**: Z
- **L**: A
- **R**: S
- **Start**: Enter
- **Select**: Backspace

## ToDos

- [ ] agb has excellent support for the [aseprite](https://www.aseprite.org/) sprite editor which can be bought for $20 or you can compile it yourself for free.

## Sprites and Tiles

The GBA renders to the screen one pixel at a time a line at a time from left to right. After it has finished rendering to each pixel of the screen, it briefly pauses rendering before starting again. This period of no drawing is called vblank, which stands for the 'vertical blanking interval'. You should `.commit()` your sprites only during this vblank phase, because otherwise you may end up moving a sprite during the rendering which could cause tearing of your objects.

### Tiles

Tiles are 8x8 pixels in size.
Background tiles, 8x8 pixel tiles are used in the background layers if they are in tile mode.

### Sprites

Gba supports 256 hardware sprites which can be from 8x8 to 64x64 pixels in size.
Also with different sizes ranging from square 8x8 to more exotic sizes like 8x32 pixels.
Each sprite can use a maximum of 16 colors out of the total sprite palette of 256 colors.

Sprites are stored in a special area of video memory called the 'Object Attribute Memory' (OAM). It stores the 'attributes' of the sprites, such as their location, whether or not they are visible, and which tile to use.
The pixel data is stored in video RAM (VRAM).

## Available agb example targets:

- no_game
- output
- dma_effect_background_colour
- animated_background
- multiple_video
- dynamic_tiles
- panic
- syscall
- test_logo
- text_render
- chicken
- windows
- affine_background
- dma_effect_circular_window
- stereo_sound
- sprites
- bitmap3
- save
- dma_effect_background_scroll
- mixer_32768
- beep
- bitmap4
- mixer_basic
- wave
- object_text_render
- allocation
