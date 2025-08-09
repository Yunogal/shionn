<p align="center">
  <img src="shionn.svg" alt="icon" width="250"/>
</p>

## Installation

```sh
git clone https://github.com/Yunogal/shionn.git
cd shionn
cargo build --release
```

## Usage

Once you have installed Shionn, you can use it like this:

```sh
# extract resource files
shionn -i example.pac

# extract
shionn example.arc

# extract
shionn example.pfs.1116

#
shionn *.ws2 -e *.exe
```

[Supported formats](https://yunogal.github.io/shionn/supported.html)
