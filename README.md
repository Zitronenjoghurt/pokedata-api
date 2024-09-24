[![Rust](https://github.com/Zitronenjoghurt/pokedata-api/actions/workflows/rust.yml/badge.svg)](https://github.com/Zitronenjoghurt/pokedata-api/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/Zitronenjoghurt/pokedata-api/graph/badge.svg?token=UM6T22YO17)](https://codecov.io/gh/Zitronenjoghurt/pokedata-api)
![](https://tokei.rs/b1/github/Zitronenjoghurt/pokedata-api?category=code&logo=https://simpleicons.org/icons/rust.svg)

# Pokedata API (work in progress)

An API which compiles pokemon data from different sources into a combined and richer format for local use.

# Setup

When built, the API will automatically download and prepare data from different data sources during the build process.
After being built, the pokemon data is bundled into the binary and is independent from all outside data sources.
If there was an update to the remote data, just delete the data directory in the project root and rebuild the app.

Automatic data updates are planned for the future.

## Pre-requisites

- Rust
- Git

## Step-by-Step

Clone the repository:

```
git clone https://github.com/Zitronenjoghurt/pokedata-api.git
```

Navigate to the project directory and then:

```
cargo run
```

Visit your preferred documentation style:

```
http://localhost:3000/docs
http://localhost:3000/rapidoc
http://localhost:3000/swagger
```

# Data storage

During the build process, Pokedata API will need to store data for local processing.

The following directories will be used ([learn more](https://docs.rs/dirs/latest/dirs/fn.data_dir.html)):

| Platform | Directory                                                |
|----------|----------------------------------------------------------|
| Linux    | `/home/{user}/.local/share/pokedata-api`                 |
| macOS    | `/Users/{user}/Library/Application Support/pokedata-api` |
| Windows  | `C:\Users\{user}\AppData\Roaming\pokedata-api`           |

! `{user}` has to be replaced by the User the API is being run on.
