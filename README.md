# Week 1 Demo: Min-Max Normalization in Rust CLI

This is a simple command-line tool implemented in Rust that performs min-max normalization on input data.

## What is Min-Max Normalization?

Min-max normalization is a data scaling technique that transforms the values of a dataset to fit within a specific range, typically between 0 and 1. The formula for min-max normalization is:

$$x_{norm} = \frac{x - min(x)}{max(x) - min(x)}$$

where $x$ is the input value, $min(x)$ is the minimum value in the dataset, $max(x)$ is the maximum value in the dataset, and $x_{norm}$ is the normalized value.

## Usage
To use this tool, simply provide a comma-separated list of numbers as input.

## Installation
To install this tool, you'll need to have Rust installed on your system. Once you have Rust installed, you can install the tool using Cargo.



## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
