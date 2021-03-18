# dc [![Build Status](https://www.travis-ci.com/ice-bit/dc.svg?branch=main)](https://www.travis-ci.com/ice-bit/dc)
`dc` is a RPN calculator written in Rust that aims to be the UNIX's dc clone. This implementation of `dc`
does supports only a proper subset of the `dc`'s features(see below for a detailed list).  

## Installation
Since `dc` is written in Rust(at least mine), you need to install a 
[Rust compiler](https://www.rust-lang.org/) in order to build this tool.
Then you can just type:

```bash
cargo build --release --verbose
```
You will find the executable file under `target/release/dc`.

## Usage
`dc` uses [reverse polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) to evaluate the expressions. For a general
list of options you can run `dc -h`:

```
dc - RPN Desk calculator 0.1.0
Marco C. <ceticamarco@gmail.com>
UNIX's dc clone

USAGE:
    dc [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    output version information and exit

OPTIONS:
    -e, --expression <expression>    evaluate expression
    -f, --file <file>                evaluate contents of file
```

Let us see some practical examples.  

1. Evaluate `(-5 + sqrt(25-16))/2`:
    ```
    -5 25 16 - v + 2 / p
    ```
    _`v` stands for square root and `p` stands for print_
2. Evaluate `(0.5+0.9)/3^4`
    ```
    0.5 0.9 + 3 4 ^ p
    ```
3. Evaluate `10 + 5 inline`:
    ```bash
    $> dc -e "10 5 +"
    ```
4. Evaluate a file full expressions`:
    ```bash
    $> cat foo
        5 5 + 
        2 d * v 
        f
    $> dc -f ./foo
    ```
5. Evalute `sin(2pi)+cos(2pi)`:(FIXME: fix this mess)
```
$> dc -e "$(dc -e '2 pi * pd') sin $(dc -e '2 pi * pd') cos + p"
```

## Features
As stated above, this tool only supports a proper subset of 
original UNIX's `dc`. Here a complete list:
| Command |        Meaning        |
|:-------:|:---------------------:|
|   `+`   |        Addition       |
|   `-`   |      Subtraction      |
|   `*`   |     Multiplication    |
|   `/`   |        Division       |
|   `%`   |         Modulo        |
|   `^`   |      Exponential      |
|   `v`   |      Square root      |
|   `c`   |      Clear stack      |
|   `d`   | Duplicate top element |
|   `p`   |   Print top element   |
|   `sin` |    Compute sine       |
|   `cos` |    Compute cosine     |
|   `tan` |    Compute tangent    |
|   `asin`|    Compute arcsine    |
|   `acos`|    Compute arccosine  |
|   `atan`|    Compute arctangent |
|   `fp`  |Print top element(converting from radiants)|

This `dc` implementation has a virtually endless stack, supports 
floating point arithmetic(standard IEE754 double precision),
inline expression parsing and file parsing.
**NOTE**: by default, all trigonometrical functions
push result in radiant, in order to print as degree, you can 
use the `fp` command.

## Unit test
To execute tests type:
```bash
cargo test --release --verbose
```


## License
This product is licensed under 
[GPLv3](https://choosealicense.com/licenses/gpl-3.0/). You will obtain a copy 
of this license by cloning this repository.
