# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/aoe2ct/aoe2rec/releases/tag/aoe2rec-tools-v0.1.0) - 2025-01-12

### Other

- Do not publish aoe2rec-js to crates.io
- Add LICENSE
- Add README
- Add other missing descriptions, do not publish API crate
- Remove macos-12 runner
- Add github workflows
- Add first draft of python module implementation
- Publish the WASM library as bundle
- Add WASM version of the library
- Add action placeholders
- Parse the operations and post game information
- Implement an API that returns the rec info
- Split project into crates for binary and library
- Add candidate implementation for 63.0 recs
- Improve alignment of variables, add enums for enhanced readability. Complete for 62.0 recs
- Remove struct from full game pattern, add instructions
- Find the next player to parse
- Add players objects
- Add the hexpat files for discovering the data format in imhex
- Add more structures and convert the result into JSON
- Add support for recs version 63.0 and above
- Add first version of the parser for the header
- Initial commit
