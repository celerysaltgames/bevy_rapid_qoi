# bevy_rapid_qoi

Bevy support for the QOI (Quite OK Image) format. Fork of [bevy_qoi](https://github.com/DigitalJokerMan/bevy_qoi) based on the [rapid_qoi](https://crates.io/crates/rapid-qoi) crate.

## Usage

Add the `QOIPlugin` to your app, and you're good to go.

```rs
use bevy::prelude::*;
use bevy_qoi::QOIPlugin;

fn main() {
    App::new()
        // ...
        .add_plugin(QOIPlugin)
        // ...
        .run();
}
```
