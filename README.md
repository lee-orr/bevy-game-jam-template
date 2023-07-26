# Game Jam Template

This is a simple bevy template providing scaffolding for creating a game jam game. It basically sets up some asset loading, menus, and some common crates for later use, as well as some VS Code tasks & Github Actions for building and deploying the game.

To see a web build of this set up - you can check it out [here](https://lee-orr.github.io/bevy-game-jam-template/)

## Crates Used

### Bevy

Using bevy, with the "flac" and "mp3" features for audio

### Leafwing Input Manager

Used for handling input in a nice, cross-control-scheme manner

### Bevy Common Assets, Bevy Asset Loader, Serde

These are used to handle asset loading, and handling of assets in yaml, json, toml or ron formats. They are also used to only load the menu once things like the fonts are available.

### Bevy Vector Shapes

This is used to draw 2d shapes on the screen - used for the loading screen (before any assets are available), as well as for some graphical elements like health bars in game.

### Bevy UI DSL & Bevy UI Navigation

These crates are used to simplify creation of UI's and handling of input.

### Bevy Inspector EGUI

This crate is used to have some default inspectors available in the game, accessed with F1.

### Bevy Turborand

Used for random number generation.

### Console Error Panic Hook

This is used in WASM builds to send panics to the console.
