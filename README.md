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



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/bernii/embedded-graphics-framebuf">
    <img src="https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/191fe7f8a0fedc713f9722b9dc59208dacadee7e/assets/logo.svg?sanitize=true" alt="Embedded graphics logo" width="80" height="80">
  </a>

<h3 align="center">Framebuffer implementation for Rust's Embedded-graphics</h3>

  <p align="center">
    Framebuffer approach helps to deal with display flickering when you update multiple parts of the display in separate operations. Intead, with this approach, you're going to write to a in-memory display and push it all at once into your hardware display when the whole picture is drawn.
    <br /><br />
    This technique is useful when you're updating large portions of screen or just simply don't want to deal with partial display updates but comes at the cost of higher RAM usage.
    <br />
    <i>The approach has been tested on TTGO (esp32) with ST7789</i>
    <br />
    <a href="https://docs.rs/embedded-graphics-framebuf/latest/embedded_graphics_framebuf/index.html"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://crates.io/crates/embedded-graphics-framebuf">Rust Crate</a>
    ·
    <a href="https://github.com/bernii/embedded-graphics-framebuf/issues">Report Bug</a>
    ·
    <a href="https://github.com/bernii/embedded-graphics-framebuf/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

This library is a Rust implementation of framebuffer approach that is often used when driving hardware displays. The goal is to perform bulk-write of all the screen pixels at once, avoiding multiple individual updates that could lead to screen flickering.

This library has been designed to work with Rust's embedded-graphics library.

<p align="right">(<a href="#top">back to top</a>)</p>



### Built With

* [rust](https://www.rust-lang.org/)
* [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics)

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

Make sure you have your `rust` environment configurated

### Installation

1. Add library to your `Cargo.toml`

    ```toml
    [dependencies]
    embedded-graphics-framebuf = "0.1.0"
    ```
2. Use the library in you code
    ```rust
    use embedded_graphics_framebuf::FrameBuf;
    ...

    let mut display = st7789::ST7789::new(
        di,
        rst.into_output()?,
        // SP7789V is designed to drive 240x320 screens, even though the TTGO physical screen is smaller
        320,
        240,
    );

    let mut fbuff = FrameBuf<Rgb565, 240_usize, 135_usize> = FrameBuf([[Rgb565::BLACK; 240]; 135]);

    fbuff.clear_black();
    Text::new(
        &"Good luck!",
        Point::new(10, 13),
        MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE.into()),
    )
    .draw(&mut fbuff).unwrap();

    display.draw_iter(fbuf.pixels()).unwrap();
    ```
3. Your flickering problems should be solved at this point :)

<p align="right">(<a href="#top">back to top</a>)</p>


<!-- ROADMAP -->
## Roadmap

- [x] add tests
- [x] add rustdocs
- [ ] CI integration with GithHub Actions
- [ ] better error generation & handling

See the [open issues](https://github.com/bernii/embedded-graphics-framebuf/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Bernard Kobos - [@bkobos](https://twitter.com/bkobos) - bkobos@gmail.com

Project Link: [https://github.com/bernii/embedded-graphics-framebuf](https://github.com/bernii/embedded-graphics-framebuf)

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* proven examlpes from [adamgreid](https://github.com/adamgreig) ([imlplementation](https://github.com/adamgreig/walkclock-public/blob/master/firmware/src/framebuf.rs ))
* [st7789](https://github.com/almindor/st7789) driver by almindor
* super helpful [embedded-graphics](https://app.element.io/#/room/#rust-embedded-graphics:matrix.org) matrix chat

<p align="right">(<a href="#top">back to top</a>)</p>



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
