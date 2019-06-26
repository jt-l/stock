Stocks
=

stock data from the command line

## install

```
git clone https://git.sr.ht/~combinations/stocks
```

## run

```
cargo run 
```

## build

```
cargo build --release
```

## configuration 

stocks was built to work with a stock data provider called worldtradingdata, to get an API key go to https://www.worldtradingdata.com/register. 

in order for the program to function you'll need to set an environment variable; ``` export WORLD_TRADING_DATA_API_KEY=<YOUR_API_KEY> ```

## usage 

```
USAGE:
    stocks [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add     Add a stock to the db
    ls      List the info for stocks in the db
    rm      Remove a stock from the db
    help    Prints this message or the help of the given subcommand(s)
```
