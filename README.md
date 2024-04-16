<h1 align="center"> ✨ Katuk ✨ </h1>

## Index 
- [Index](#-index)
- [Introduction](#-introduction)
- [Installation](#-installation)
- [Usage](#-usage)

## Introduction 

Directory Bookmarking CLI written in rust 

## Installation 

### for Fish 

1. clone this repository 
2. cd to the repository 
3. copy target/release/katuk_rs to your PATH
5. write this command to the end of your fish config file (~/.config/config.fish) 
```bash 
source /path/to/this/repo/katuk
```
6. enjoy :D



## Usage

```bash
-n <name> # new bookmark for current directory 
-n <name> <path>  # new bookmark for path 
-p <name> # get the path 
-d <name> # delete the path
-l # list all the bookmarks
```
