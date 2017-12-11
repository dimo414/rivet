# Rivet

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

## Coursework

* [Proposal](https://docs.google.com/document/d/1a3i-FbnnbSmXzR1A1wHFs1Z0LbnrB-3k_TgvPvc4By0/edit)
* [Checkpoint](https://docs.google.com/document/d/1U-t16dSLtTCSrh781-u-_Os_tzxjF1PWpSl0ByN-wDY/edit)
