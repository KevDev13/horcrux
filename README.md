# Horcrux

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![CI](https://github.com/KevDev13/horcrux/workflows/CI/badge.svg?branch=main)
![GitHub repo size](https://img.shields.io/github/repo-size/KevDev13/horcrux?style=plastic)
![GitHub last commit](https://img.shields.io/github/last-commit/KevDev13/horcrux?color=green&style=plastic)

Horcrux is a CLI utility that will take a file and use Shamir's Secret Sharing Scheme to break it into multiple files. It is still a work in progress, and has some problems still, so use at your own risk.

## Usage

**Currently, Horcrux only works for text files. This is planned to be updated before v1.0**

### Splitting a File

To split a file, after building, run `./horcrux -s [filename] [minimum shares] [total shares]`

Example:
```
./horcrux -s test.txt 3 5
```
The above will splt the file test.txt into 5 total shares, requiring at least 3 of them to recover the information. The share files will be called `share1`, `share2`, etc.

### Recovering a file

To recover a file, after building, run `./horcrux -r [filename] [share_file] [share_file]...` (list all share files after filename)

Example:
```
./horcrux -r output.txt share1 share3 share5
```
The above will recover the secret file into output.txt from the share files listed after output.txt. If not enough shares are present, the file will not be created.

NOTE: when splitting a file, a file named `header.txt` is created. This file is **required** in order to recover the secret. [An issue](https://github.com/KevDev13/horcrux/issues/5) as been opened to no longer require this file, but for now it is required. Sorry.

## Other Information

For those wondering where the name came from, Horcrux is an item from the Harry Potter series of books (spoliers in the link): https://en.wikipedia.org/wiki/Magical_objects_in_Harry_Potter#Horcruxes
