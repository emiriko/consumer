<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-light.svg">
    <img alt="The Rust Programming Language: A language empowering everyone to build reliable and efficient software"
         src="https://raw.githubusercontent.com/rust-lang/www.rust-lang.org/master/static/images/rust-social-wide-light.svg"
         width="50%">
  </picture>

[Website][Rust] | [Getting started] | [Learn] | [Documentation] | [Contributing]
</div>

This is the main source code repository for [Rust]. It contains the compiler,
standard library, and documentation.

[Rust]: https://www.rust-lang.org/
[Getting Started]: https://www.rust-lang.org/learn/get-started
[Learn]: https://www.rust-lang.org/learn
[Documentation]: https://www.rust-lang.org/learn#learn-use
[Contributing]: CONTRIBUTING.md

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    ROCKET_PORT=8001
    APP_INSTANCE_ROOT_URL=http://localhost:${ROCKET_PORT}
    APP_PUBLISHER_ROOT_URL=http://localhost:8000
    APP_INSTANCE_NAME=Safira Sudrajat
    ```
    Here are the details of each environment variable:
    | variable                | type   | description                                                     |
    |-------------------------|--------|-----------------------------------------------------------------|
    | ROCKET_PORT             | string | Port number that will be listened by this receiver instance.    |
    | APP_INSTANCE_ROOT_URL   | string | URL address where this receiver instance can be accessed.       |
    | APP_PUUBLISHER_ROOT_URL | string | URL address where the publisher instance can be accessed.       |
    | APP_INSTANCE_NAME       | string | Name of this receiver instance, will be shown on notifications. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)
3.  To simulate multiple instances of receiver,
    you can open new terminal, then edit `ROCKET_PORT` in `.env` file, then execute another `cargo run`.

    For example, if you want to run 3 (three) instances of BambangShop Receiver at port `8001`, `8002`, and `8003`, you can do these steps:
    -   Edit `ROCKET_PORT` in `.env` to `8001`, then execute `cargo run`.
    -   Open new terminal, edit `ROCKET_PORT` in `.env` to `8002`, then execute `cargo run`.
    -   Open another new terminal, edit `ROCKET_PORT` in `.env` to `8003`, then execute `cargo run`.
      
## Why Rust?

- **Performance:** Fast and memory-efficient, suitable for critical services, embedded devices, and easily integrate with other languages.

- **Reliability:** Our rich type system and ownership model ensure memory and thread safety, reducing bugs at compile-time.

- **Productivity:** Comprehensive documentation, a compiler committed to providing great diagnostics, and advanced tooling including package manager and build tool ([Cargo]), auto-formatter ([rustfmt]), linter ([Clippy]) and editor support ([rust-analyzer]).

[Cargo]: https://github.com/rust-lang/cargo
[rustfmt]: https://github.com/rust-lang/rustfmt
[Clippy]: https://github.com/rust-lang/rust-clippy
[rust-analyzer]: https://github.com/rust-lang/rust-analyzer

## Quick Start

Read ["Installation"] from [The Book].

["Installation"]: https://doc.rust-lang.org/book/ch01-01-installation.html
[The Book]: https://doc.rust-lang.org/book/index.html

## Installing from Source

If you really want to install from source (though this is not recommended), see
[INSTALL.md](INSTALL.md).

## Getting Help

See https://www.rust-lang.org/community for a list of chat platforms and forums.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Rust is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.

## Trademark

[The Rust Foundation][rust-foundation] owns and protects the Rust and Cargo
trademarks and logos (the "Rust Trademarks").

If you want to use these names or brands, please read the
[media guide][media-guide].

Third-party logos may be subject to third-party copyrights and trademarks. See
[Licenses][policies-licenses] for details.

[rust-foundation]: https://foundation.rust-lang.org/
[media-guide]: https://foundation.rust-lang.org/policies/logo-policy-and-media-guide/
[policies-licenses]: https://www.rust-lang.org/policies/licenses
