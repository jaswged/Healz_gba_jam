# Healz

Quest through a dungeon playing as the healer in a party of 4 in this GBA game.
Party consists of a sword and board tank, lightning wizard, bow ranger and you!

## ToDo

- [x] Create characters with health and dps logic
- [x] Create sprites for characters
- [x] Create a single heal spell to test health system
- [x] Create a boss to attack characters
- [x] Have chars dps back with
  - [x] Animations for unique attacks
- [x] Game over screen
- [x] Mana bar and boss AOE cooldown
- [x] Text writer for player dialog or story
- [x] Cooldowns for spells and boss abilities
- [x] UI Text for spells and names
- [x] Spell effects
- [x] Character animations for abilities
- [x] Sound effects and music
- [ ] ...
- [ ] everything else

### Sprite ToDos

- [x] Big bad boss guy
- [x] Figure out how to do text. is it all char sprites or some tool
- [x] Spell effects
- [x] Multiple bosses
- [ ] ...

### Nice To Haves

- [ ] Intro battle for warming up to the real deal, like a tutorial?
- [x] Pause menu for how to play on `Start/Select` buttons
- [ ] Keep track of total healing and over-healing so at the end you can share those stats.
- [ ] Rogue-like dungeons with more packs and bosses
- [x] Perhaps a story for the whole dungeon
- [ ] ~~Gear found on boss to make later fights easier (Lewt)~~
- [ ] ...

## Gameplay

Each character has a set dps and health.

## Characters

todo!();

E.G.

| Character | Health  | DPS       | About                       |
|-----------|---------|-----------|-----------------------------|
| Tank-Man  | 20      | 2         | Knight from the royal guard |
| Whizard   | 12      | 4         | Wise and powerful           |
| Rangeer   | 16      | 3         | Skilled hunter              |
| Healz     | 16      | 3         | Cleric devoted to healing   |
| Boss Boi  | 100     | 5dps/3dps | Evil lich or something      |

Boss Ideas:

- Corrupted Priest/Wizard
- Dragon
- Skeleton
- Lich
- Shadow wraith

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
So with 16px tiles you get 15 x 10 tiles or 30 x 20 with 8px by 8px

Frame around selected character
Γ    ˥
L    ˩

## Assets

- corners.aseprite" 16x16
- buttons.aseprite" 16x16
- boss_hp.aseprite" 16x16
- characters.aseprite" 32x32
- bosses.aseprite" 64x64
- banner.aseprite" 32x32
- health.aseprite" 8x8
- spell_effects.aseprite" 16x16 hourglass and mini crab

### Backgrounds

Backgrounds are divided into 4 layers.

0: Splash screens such as title or game overs 
1: Spell names and health bar outlines
2: Character names and book banner
3: Terrain locations

## Building AGB

### Prerequisites

You will need the following installed in order to build and run this project:

* A recent version of `rustup`. See the [rust website](https://www.rust-lang.org/tools/install) for instructions for your operating system

You will also want to install an emulator. The best support in agb is with [mgba](https://mgba.io), with
`println!` support via `agb::println!` but any emulator should work. You'll get the best experience if
`mgba-qt` is in your `PATH`.

### Running in an emulator

Once you have the prerequisites installed, you should be able to build using

```sh
cargo build
```

The resulting file will be in `target/thumbv4t-none-eabi/debug/<your game>` or `target/thumbv4t-none-eabi/release/<your game>` depending on
whether you did a release or debug build.

If you have `mgba-qt` in your path, you will be able to run your game with

```sh
cargo run
```

## Shipping a .gba file for real hardware

If you want to run your game on real hardware, you will also need to install `agb-gbafix` which you can do after installing
rust with the following: `cargo install agb-gbafix`. This is not required if you are only running your game in an emulator.

First build the binary in release mode and then do the following:

ToDo: Works on omega and iOs delta, but fails on GPSP.

```sh
cargo build --release; agb-gbafix --debug target/thumbv4t-none-eabi/release/Healz -o Healz_jam.gba
```
