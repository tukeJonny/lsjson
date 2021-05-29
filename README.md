# lsjson

[![build](https://github.com/tukeJonny/lsjson/actions/workflows/release.yml/badge.svg)](https://github.com/tukeJonny/lsjson/actions/workflows/release.yml)
[![test](https://github.com/tukeJonny/lsjson/actions/workflows/test.yml/badge.svg)](https://github.com/tukeJonny/lsjson/actions/workflows/test.yml)

## What

lsjson prints json path from json file.

if there are large json file, this tool is helpful for understanding `What paths exist for this json?` .

## Usage

```
$ lsjson -h
lsjson 0.1.0

USAGE:
    lsjson [FLAGS/OPTIONS] [<file-path>]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <file-path>    the path to json file
```

## How to use

Simply, below example lists all keys of json.

```
$ lsjson example.json
.glossary
.glossary.GlossDiv
.glossary.GlossDiv.GlossList
.glossary.GlossDiv.GlossList.GlossEntry
.glossary.GlossDiv.GlossList.GlossEntry.Abbrev
.glossary.GlossDiv.GlossList.GlossEntry.Acronym
.glossary.GlossDiv.GlossList.GlossEntry.GlossDef
.glossary.GlossDiv.GlossList.GlossEntry.GlossDef.GlossSeeAlso
.glossary.GlossDiv.GlossList.GlossEntry.GlossDef.para
.glossary.GlossDiv.GlossList.GlossEntry.GlossSee
.glossary.GlossDiv.GlossList.GlossEntry.GlossTerm
.glossary.GlossDiv.GlossList.GlossEntry.ID
.glossary.GlossDiv.GlossList.GlossEntry.SortAs
.glossary.GlossDiv.title
.glossary.title
```

## Install

Please download binaries from [Release](https://github.com/tukeJonny/lsjson/releases) or clone this repository && `cargo install` .


## Todo

* --stdin ... input json content from stdin
* --search-key ... print key includes specified key
* --search-value ... print key has specified value
* --values ... print all keys has scalar value
* --tree ... print all keys with tree-format
