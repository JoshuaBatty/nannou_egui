# nannou_egui
[![Actions Status](https://github.com/alexene/nannou_egui/workflows/nannou_egui/badge.svg)](https://github.com/alexene/nannou_egui/actions) [![Crates.io](https://img.shields.io/crates/v/nannou_egui.svg)](https://crates.io/crates/nannou_egui) [![Crates.io](https://img.shields.io/crates/l/nannou_egui.svg)](https://github.com/alexene/nannou_egui/blob/master/LICENSE-MIT) [![docs.rs](https://docs.rs/nannou_egui/badge.svg)](https://docs.rs/nannou_egui/)

This is my [egui] integration for nannou. The purpose of this is to allow you to tune values for your generative art creations without requiring a compilation cycle.

There are a bunch of rough edges as this is really early in dev (and I am not familiar with webgpu).
Most notably, right now you need to have MSAA = 1 in your window settings and scaling doesn't work at the moment.
For inspiration on how to expose UI widgets, please check the [egui] repo as it has a lot of examples. You have sliders, color pickers, checkboxes, dropdownlists and many more widgets available.

For information on how to integrate it to your nannou creations, there's an [example] in this repo.

To run the circle packing example: `cargo run --example circle_packing`:

![](https://github.com/AlexEne/nannou_egui/blob/main/nannou_egui/media/circle_packing.gif)


To run the color tune example: `cargo run --example tune_color`:

![](https://github.com/AlexEne/nannou_egui/blob/main/nannou_egui/media/tune_egui.gif)

## Todo
- [ ] Easier integration for storing tunable parameters to disk.
- [ ] Shortcuts for hiding the UI

[egui]: https://github.com/emilk/egui
[example]: https://github.com/AlexEne/nannou_egui/tree/main/nannou_egui_example
