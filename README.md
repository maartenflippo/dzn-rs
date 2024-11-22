# DZN-RS

A Rust library for reading [DZN](https://www.minizinc.org/doc-latest/en/spec.html#spec-model-instance-files) files.
The documentation can be found on https://docs.rs/dzn-rs.

The goal of the library is to be able to parse _all_ DZN files correctly. 
Before that there will not be a 1.0 release.
However, the library may be useful for many DZN files without being able to parse every DZN file.

## To-Do

- [-] Primitive Values
  - [x] Boolean
  - [x] Integer
  - [ ] Float
- [ ] Sets
  - [ ] Boolean
  - [ ] Integer
  - [ ] Float
- [-] Arrays (1d, 2d)
  - [x] Boolean
  - [x] Integer
  - [ ] Float
  - [ ] Set

## Installation

To add this library as a dependecy to your project, run

```
cargo add dzn-rs
```

## Contributing

I am open to receiving pull requests if you identify bugs or want to add features.
If you are motivated to do so, please open an issue first.

