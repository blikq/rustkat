README
## Introduction

This code is a simple program written in Rust that reads the contents of a file and prints them to the terminal. Here is a documentation and installation guide for running the code on Ubuntu:

## Prerequisites

Rust programming language, you can install it by running the following command:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Make sure rust is installed by running the command rustc --version

  

## Installation

Clone the repository containing the code to your local machine using git:

     git clone https://github.com/<username>/<repository>.git

. Navigate to the directory where the code is located using the command `cd <directory>`

. Build the code using the command `cargo build --release`

  

## Usage

Run the program using the command ./target/release/<executable>

  

When prompted, enter the file path for the file you want to read and print the contents of.

  

Alternatively, you can also use the following command to run the program and pass the file path as an argument to the program
  
`./target/release/<executable> <file path>`

Note: you might need to add execute permission to the file by running `chmod +x <executable>`,
rustkat app in the main directory is compiled for ubuntu
and remember its case sensitive
