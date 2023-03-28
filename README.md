# Rivet

## About

CS 242: Programming Languages [Final Project](https://stanford-cs242.github.io/f17/project/) for
Fall Quarter, 2017. We explored different approaches to the Strategy Pattern and Dependency
Injection in Rust, and authored a
[paper](https://drive.google.com/open?id=1Ozn7aZeAT36PbBXAtLVjQQeuLVu7E-Pe) of our findings.

Authors: Michael Diamond and Matthew Vilim

## Running

```shell
# launch server
$ cargo run
```

Access server at http://localhost:8000

If Ctrl+C isn't sufficient to kill the server (this seems to be the case on Windows) visit
[/quit](http://localhost:8000/quit) to kill the server.

Run `./test_server.sh` to valdidate the server's runtime behavior (namely, that it doesn't panic).

## Resources

* [`tiny_http` docs](https://tiny-http.github.io/tiny-http/tiny_http)
  * [example server](https://github.com/tomaka/example-tiny-http)
* [Project Notes](https://docs.google.com/document/d/182-uPnD8Jd7VNaU7A7t2rDrBSeip0EU4H3IenELNcOY/edit)
* [Rocket Codegen](https://api.rocket.rs/rocket_codegen/)
* [Build Scripts for Rust](http://doc.crates.io/build-script.html)

### DI Frameworks

* https://github.com/KodrAus/rust-ioc
* https://github.com/jonysy/hypospray
  * https://docs.rs/hypospray/0.1.2/hypospray
* https://github.com/Nercury/di-rs

## License

Licensed under the Apache 2 license, copyright 2017 Google, Matthew Vilim.

This is not an officially supported Google product.
