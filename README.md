# Introduction

This repository is based on the book Zero To Production In Rust: An introduction to backend development by Luca Palmieri.

# Notes

## How to run integration tests

### First start Postgres container

```sh 
$ pushd tools/db && sudo docker build . -t zerotoproddb
$ popd
$ sudo docker run -p 5432:5432 -t zerotoproddb
```
