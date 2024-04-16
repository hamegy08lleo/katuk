<h1 align="center"> ✨ Katuk ✨ </h1>

## Introduction 

Directory Bookmarking CLI written in rust 

## Installation 

### Prerequisites 

- cargo 
- rustc

You can install it from [here](https://www.rust-lang.org)

### For Fish 

1. clone this repository 
2. cd to the repository 
3. build the code with this command
```
cargo build --release --bin katuk_rs
```
4. copy target/release/katuk_rs to your PATH (e.g., usr/bin/)
5. write this command to the end of your fish config file (~/.config/config.fish) 
```bash 
source /path/to/this/repo/katuk
```
6. enjoy :D

## Usage

```bash
-a <name> # new bookmark for current directory 
-a <name> <path>  # new bookmark for path 
-p <name> # go to the path 
-d <name> # delete the bookmark
-e <name> <path> edit the bookmark
-l # list all the bookmarks
```
