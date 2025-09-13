<h1 align="center">crlfhash</h1>

## Description

Calculate the hashes of a file (in memory) with different line endings (e.g., with and without carriage returns).

This is particularly useful for calculating [IOCs](https://en.wikipedia.org/wiki/Indicator_of_compromise) of scripts that are functionally equivalent but may have differing line endings depending on system they were developed on.

Line endings with carriage returns are commonly known as [CRLF](https://stackoverflow.com/a/1552775/), hence the project name.

## Quick Start

```bash
$ just
$ ./crlfhash -h
```

## Similar Projects

The earlier sister package to `crlfhash` is [`xmemhash`](https://github.com/jakewilliami/crlfhash), which computes the hashes of the contents of archives, without writing the contents to disk.
