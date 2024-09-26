# Guide

These indices are generated using the Pokedata API CLI. The default indices are already provided, if you don't want to
change anything or build indices for self-hosting you can keep them as is.

## Step-by-Step

---
Firstly, the project has to be built so the CLI is fully ready. Use this command in the project root. Remember, you will
need to have Rust installed.

```
cargo build --release
```

---
After this, you can call CLI commands using:

```
./cli
```

---
! The following command will clone the official PokeAPI sprites repo which is a few GB in size, this can take a few
minutes when run for the first time (depending on your internet connection).

To build all necessary sprite indices just use:

```
./cli pokeapi build-sprites
```

---
Alternatively, if you want to host the sprites yourself, you can add the optional self-host flag:

```
./cli pokeapi build-sprites --self-host
```

This will copy all sprites with a tokenized filename into ./data/pokemon-sprites-self-host

---
Lastly, just copy the generated indices into ./sprite_indices and you are all done.