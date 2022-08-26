# bevy_rapid_qoi

[![Latest Version](https://img.shields.io/crates/v/bevy_rapid_qoi.svg)](https://crates.io/crates/bevy_rapid_qoi)
[![Rust Documentation](https://docs.rs/bevy_rapid_qoi/badge.svg)](https://docs.rs/bevy_rapid_qoi)
![Crates.io](https://img.shields.io/crates/l/bevy_rapid_qoi)
![Crates.io](https://img.shields.io/crates/d/bevy_rapid_qoi)

Bevy support for the QOI (Quite OK Image) format. Fork of [bevy_qoi](https://github.com/DigitalJokerMan/bevy_qoi) based on the [rapid_qoi](https://crates.io/crates/rapid-qoi) crate.

## Features

- Enable Bevy to load QOI assets with the `QOIAssetLoader`.
- Implement a `QOIPlugin` to register the asset loader more conveniently.

## Quickstart

Add the `QOIPlugin` to your app, and you're good to go.

```rs
use bevy::prelude::*;
use bevy_rapid_qoi::QOIPlugin;

fn main() {
    App::new()
        .add_plugin(QOIPlugin)
        .run();
}
```

## Compatibility matrix

| Bevy | bevy_rapid_qoi |
|------|----------------|
| 0.8  | 1.0            |

## Inspired by

- The [Bevy Qoi](https://crates.io/crates/bevy_qoi) plugin.
- The excellent [Rapid QOI](https://crates.io/crates/rapid-qoi) implementation of QOI in Rust.
