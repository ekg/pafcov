# pafcov: PAF Coverage Calculator

This repository contains a Rust program that calculates the coverage of a target sequence in a PAF file. The program can calculate coverage based on total mapping overlap or base-by-base alignment. The output is in BED format.

## Build

To build the program, you will need the Rust programming language installed. You can install Rust using [rustup](https://rustup.rs/).

Once Rust is installed, you can build the program by cloning this repository and running `cargo build`:

```sh
git clone https://github.com/your-username/paf-coverage-calculator.git
cd paf-coverage-calculator
cargo build
```

This will create an executable file in the `target/debug` directory.

You can also install `pafcov` in your environment:

```sh
cargo install --force --path .
```

## Usage

To use the program, you will need a PAF file as input. A PAF file is a text file that contains information about alignments between query sequences and target sequences. You can find more information about PAF files [here](https://github.com/paftools/paftools/blob/master/doc/PAF.md).

Once you have a PAF file, you can run the program with the following command:

```sh
cargo run --release -- --input test/input.paf
```

Given this input:

```txt
query1	100	0	30	+	target1	1000	100	130	cg:z:10M10X10=
query2	100	0	30	+	target1	1000	105	135	cg:z:10M10X10=
```

The output will contain a coverage map relative to `target1`:

```
...
target1 99      100     0
target1 100     101     1
target1 101     102     1
target1 102     103     1
target1 103     104     1
target1 104     105     1
target1 105     106     2
target1 106     107     2
target1 107     108     2
target1 108     109     2
target1 109     110     2
target1 110     111     1
target1 111     112     1
target1 112     113     1
target1 113     114     1
target1 114     115     1
target1 115     116     0
target1 116     117     0
target1 117     118     0
...
```

For typical use, replace `input.paf` with the name of your PAF file. The program will calculate the coverage of each target sequence in the PAF file and output the results in BED format.

Characters which the cigar asserts to be `=` will be counted in the coverage map. But this behavior can be adjusted with the `-o/--overlap` flag, which counts any overlapping mapping in the coverage count.

## Options

You can use the following options when running `pafcov`:

```
  -i, --input <INPUT>  Input PAF file
  -o, --overlap        Calculate coverage based on total overlap
  -h, --help           Print help
  -V, --version        Print version
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
