# hashdistinct
It is simple utility for deletion duplications.

## Installation
```
cargo install hashdistinct
```

## Usage
It is simple as ls. Just look at the usage:

```
Distinct Hash 0.3.3
Starfear https://github.com/starfear
Utility for deletion duplications with same hash.

USAGE:
    hashdistinct [FLAGS] [OPTIONS] <targets>...

FLAGS:
    -h, --help       Prints help information
    -s, --silent     silent mode
    -V, --version    Prints version information

OPTIONS:
    -a, --algorithm <algorithm>    hash algorithm. Supported algorithms: [SHA256, SHA384, SHA512, SHA512_256]

ARGS:
    <targets>...    targets
```

## Usage examples
```
# raw
hashdistinct foo.webm bar.webm too.webm

# for sh, bash zsh
hashdistinct webms/*

# specify algorithm (SHA256, SHA384, SHA512, SHA512_256)
hashdistinct webms/* -a SHA512_256

# if you prefer silent mode
hashdistinct webms/* -a SHA512_256 --silent
```

## Contribute
Seems like this software doesn't have to be improved, but I don't mind to add support for md5 and other non-ring algorithms.
Pull requests and issues are welcome.

## License
Under GNU/GPL3