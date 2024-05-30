# Healz

Quest through a dungeon playing as the healer in a party of 4 in this GBA game.
Party consists of a sword and board tank, lightning wizard, bow ranger and you!

## ToDo

- [ ] Create characters with health and dps logic
- [ ] Create sprites for characters
- [ ] Create a single heal spell to test health system
- [ ] Create a boss to attack characters
- [ ] Have chars dps back with
  - [ ] Animations for unique attacks
- [ ] ...
- [ ] ...
- [ ] ...
- [ ] ...
- [ ] ...
- [ ] everything else

### Sprite ToDos

- [ ] Get 4 party member characters
- [ ] Big bad boss guy
- [ ] Health bar. Mana bar, Boss bar with skull. use existing one?
- [ ] Figure out how to do text. is it all char sprites or some tool
- [ ] Spell effects
- [ ] Maybe make boss health bar 12-16 px high so you can fit a skull
- [ ] ...
- [ ] ...
- [ ] ...
- [ ] ...

### Nice To Haves

- [ ] Intro battle for warming up to the real deal, like a tutorial?
- [ ] Keep track of total healing and over-healing so at the end you can share those stats.
- [ ] Rogue-like dungeons with more packs and bosses
- [ ] Perhaps a story for the whole dungeon
- [ ] StS style overworld going node to node
- [ ] Gear found on boss to make later fights easier
- [ ] Xp for level ups
- [ ] Save game logic
- [ ] ...

## Gameplay

`todo!();`

Each character has a set dps and health.

E.G.

| Character | Health  | DPS       |
|-----------|---------|-----------|
| Tank-Man  | 20      | 2         |
| Whizard   | 12      | 4         |
| Rangeer   | 16      | 3         |
| Boss Boi  | 100     | 5dps/3dps |

### Controls

- A: 1 sec cast basic heal `Bandage`
- B: Instant cast heal with a cooldown `Cauterize`
- L: Hot heals over 5 seconds. Only 1 at a time `Regenerate`
- R: Hold to generate `Meditate`
- D-pad: Move cursor around to select which character to cast spell on.
- Start: Start over?
- Select: Menu shows details for above spells

## UI Layout

GBA is 240x160 pixels.
So with 16px tiles you get 15 x 10 tiles

Frame around selected character
Γ    ˥
L    ˩

## Story

Game begins with dialog,
> One last pack and we're at the boss. Is everyone ready?

A DBM type ready check sounds and player presses `A` to begin the pull.

This first combat is a tutorial of sorts. (Skip for POC. just boss)

Final boss is a big dragon/cat that has 2 phases and 2 attacks. Single target on tank and a cleave for everyone.

## Building AGB

### Prerequisites

You will need the following installed in order to build and run this project:

* A recent version of `rustup`. See the [rust website](https://www.rust-lang.org/tools/install) for instructions for your operating system

You will also want to install an emulator. The best support in agb is with [mgba](https://mgba.io), with
`println!` support via `agb::println!` but any emulator should work. You'll get the best experience if
`mgba-qt` is in your `PATH`.

If you want to run your game on real hardware, you will also need to install `agb-gbafix` which you can do after installing
rust with the following: `cargo install agb-gbafix`. This is not required if you are only running your game in an emulator.

### Running in an emulator

Once you have the prerequisites installed, you should be able to build using

```sh
cargo build
```

or in release mode (recommended for the final version to ship to players)

```sh
cargo build --release
```

The resulting file will be in `target/thumbv4t-none-eabi/debug/<your game>` or `target/thumbv4t-none-eabi/release/<your game>` depending on
whether you did a release or debug build.

If you have `mgba-qt` in your path, you will be able to run your game with

```sh
cargo run
```

or in release mode

```sh
cargo run --release
```

## Shipping a .gba file for real hardware

To make a game run on real hardware, you will need to convert the built file into a file suitable for
running on the real thing.

First build the binary in release mode using the instructions above, then do the following:

```sh
agb-gbafix target/thumbv4t-none-eabi/release/<your game> -o <your game>.gba
```
