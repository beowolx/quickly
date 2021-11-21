# Quickly CLI 📰 💻 🚀

## Description 📚

`quickly` is a command line tool build with Rust that lets you search for headlines news based in a country, category and query.

It uses the [News API](https://newsapi.org/) to fetch the headlines from sources
and blogs around the world.

Currently supported countries: 🇧🇷 🇫🇷 🇬🇧 🇺🇸

![screenshot of gh pr status](./assets/carbon.png)

## Installation 👷‍♀️

First of all, you'll need a [News API](https://newsapi.org/) key.
You can get one from their website, it is **free**.

#### Clone the repository

```
$ git clone https://github.com/LuisCardosoOliveira/quickly.git
$ cd quickly
```

Once finished, go ahead and create an `.env` file with our API_KEY like that (replacing the `<key>` by your real key):

```
API_KEY=<key>
```

#### Supposing that you already have Rust installed, compile the package

```
$ cargo build --release
```

#### Run the program

```
$ ./target/debug/quickly <COUNTRY> <CATEGORY> <QUERY>
```

#### Requirements

[Rust](https://www.rust-lang.org/tools/install)

## Usage

### Instructions

```
$ ./target/debug/quickly --help

Quickly 0.1.0

Luis C. <luis@luiscardoso.dev>

Quickly is a simple CLI that lets you search for headlines news.

USAGE:
    quickly [ARGS]

ARGS:
    <COUNTRY>     The 2-letter ISO 3166-1 code of the country you want to get headlines for.
                  [default: us] [possible values: br, fr, gb, us]
    <CATEGORY>    The category you want to get headlines for. [default: technology] [possible
                  values: business, entertainment, general, health, science, sports, technology]
    <QUERY>       Keywords or a phrase to search for.

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

### Default search (US, Technology)

```
$ ./target/debug/quickly
```

### Search for headlines with the query Apple

```
$ ./target/debug/quickly -- Apple
```

### Search headlines in France with category settled as technology and query Black Friday

```
$ ./target/debug/quickly fr technology -- Black Friday
```

## Contributing 🤖

If anything feels off, or if you feel that some functionality is missing, feel
free to contribue.

## License

MIT © [Luis Cardoso](https://twitter.com/LuisFCCO)
