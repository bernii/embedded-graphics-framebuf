<div id="top"></div>

<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![Build Status][build-status]][build-status-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]


<br />
<div align="center">
  <a href="https://github.com/bernii/embedded-graphics-framebuf">
    <img src="https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/191fe7f8a0fedc713f9722b9dc59208dacadee7e/assets/logo.svg?sanitize=true" alt="Embedded graphics logo" width="80" height="80">
  </a>

<h2 align="center">Framebuffer implementation for Rust's Embedded-graphics</h3>
  <p align="center">
    <a href="https://docs.rs/embedded-graphics-framebuf/latest/embedded_graphics_framebuf/index.html"><strong>Documentation</strong></a>
    <br />
    <br />
    <a href="https://crates.io/crates/embedded-graphics-framebuf">Crates.io</a>
    ·
    <a href="https://github.com/bernii/embedded-graphics-framebuf/issues">Report a Bug</a>
    ·
    <a href="https://github.com/bernii/embedded-graphics-framebuf/issues">Feature Request</a>
  </p>
</div>


## About The Project

This [Rust](https://www.rust-lang.org/) library is an implementation of the framebuffer approach for the [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) ecosystem. The goal is to perform bulk-write of all the screen pixels at once, instead of having multiple individual updates for separate primitives.

Graphic compositing in multiple operations with direct updates on a display can lead to flickering and clearing previous content on the screen is harder. A Framebuffer helps to deal with this by drawing on an in-memory display, so the final display image can be pushed all at once to your hardware display.

This technique is useful when you're updating large portions of screen or just simply don't want to deal with partial display updates but comes at the cost of higher RAM usage and more traffic to the displays. This crate also has DMA support, which can enhance the performance of larger display updates.


## Getting Started

Make sure you have your `rust` environment configurated

### Installation

1. Add library to your `Cargo.toml`

    ```toml
    [dependencies]
    embedded-graphics-framebuf = "0.2.0"
    ```
2. Use the library in you code
    ```rust
    use embedded_graphics_framebuf::FrameBuf;

    // ...

    // Backend for the buffer
    let mut data = [BinaryColor::Off; 12 * 11];
    let mut fbuf = FrameBuf::new(&mut data, 12, 11);

    // You would use a "real" display here...
    let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
    Line::new(Point::new(2, 2), Point::new(10, 2))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
        .draw(&mut fbuf)
        .unwrap();
    // Write it all to the display
    display.draw_iter(fbuf.into_iter()).unwrap();
    ```
3. Your flickering problems should be solved at this point :)


## Roadmap

- [x] add tests
- [x] add rustdocs
- [x] CI integration with GithHub Actions
- [ ] better error generation & handling

See the [open issues](https://github.com/bernii/embedded-graphics-framebuf/issues) for a full list of proposed features (and known issues).


## License

Distributed under the MIT License. See `LICENSE` for more information.


## Contact

Bernard Kobos - [@bkobos](https://twitter.com/bkobos) - bkobos@gmail.com

Jounathaen - jounathaen at mail dot de

Project Link: [https://github.com/bernii/embedded-graphics-framebuf](https://github.com/bernii/embedded-graphics-framebuf)

## Acknowledgments

* proven examlpes from [adamgreid](https://github.com/adamgreig) ([imlplementation](https://github.com/adamgreig/walkclock-public/blob/master/firmware/src/framebuf.rs ))
* [st7789](https://github.com/almindor/st7789) driver by almindor
* super helpful [embedded-graphics](https://app.element.io/#/room/#rust-embedded-graphics:matrix.org) matrix chat


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/bernii/embedded-graphics-framebuf.svg?style=for-the-badge
[contributors-url]: https://github.com/bernii/embedded-graphics-framebuf/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/bernii/embedded-graphics-framebuf.svg?style=for-the-badge
[forks-url]: https://github.com/bernii/embedded-graphics-framebuf/network/members
[stars-shield]: https://img.shields.io/github/stars/bernii/embedded-graphics-framebuf.svg?style=for-the-badge
[stars-url]: https://github.com/bernii/embedded-graphics-framebuf/stargazers
[issues-shield]: https://img.shields.io/github/issues/bernii/embedded-graphics-framebuf.svg?style=for-the-badge
[issues-url]: https://github.com/bernii/embedded-graphics-framebuf/issues
[license-shield]: https://img.shields.io/github/license/bernii/embedded-graphics-framebuf.svg?style=for-the-badge
[license-url]: https://github.com/bernii/embedded-graphics-framebuf/blob/main/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/bernii
[product-screenshot]: images/screenshot.png
[build-status]: https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fbernii%2Fembedded-graphics-framebuf%2Fbadge%3Fref%3Dmain&style=for-the-badge
[build-status-url]: https://actions-badge.atrox.dev/bernii/embedded-graphics-framebuf/goto?ref=main
