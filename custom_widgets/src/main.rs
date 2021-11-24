mod toggle_switch;
mod custom_slider;
mod mb_slider;

use custom_slider::*;
use toggle_switch::*;

use mb_slider::*;

use nannou::prelude::*;
use nannou_egui::{egui, Egui};

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 360.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    egui: Egui,
    radius: f32,
    color: Hsv,
    custom_switch_bool: bool,
    lfo1: f64,
    lfo2: f64,
    lfo3: f64,
    lfo4: f64,
    lfo5: f64,
}

fn model(app: &App) -> Model {
    // Create a new window! Store the ID so we can refer to it later.
    let window_id = app
        .new_window()
        .title("Nannou + Egui")
        .size(WIDTH as u32, HEIGHT as u32)
        .raw_event(raw_window_event) // This is where we forward all raw events for egui to process them
        .view(view) // The function that will be called for presenting graphics to a frame.
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();

    Model {
        egui: Egui::from_window(&window),
        radius: 40.0,
        color: hsv(10.0, 0.5, 1.0),
        custom_switch_bool: false,
        lfo1: 0.0,
        lfo2: 0.0,
        lfo3: 0.0,
        lfo4: 0.0,
        lfo5: 0.0,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut radius,
        ref mut color,
        ref mut custom_switch_bool,
        ..
    } = *model;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    let mut fonts = egui::FontDefinitions::default();
    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert("my_font".to_owned(),
    std::borrow::Cow::Borrowed(include_bytes!("../assets/Arial/arial_bold.ttf"))); // .ttf and .otf supported

    // Put my font first (highest priority):
    fonts.fonts_for_family.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, "my_font".to_owned());

    ctx.set_fonts(fonts);

    egui::Window::new("EGUI window")
        .default_size(egui::vec2(0.0, 200.0))
        .frame(egui::containers::Frame {
            fill: egui::Color32::from_gray(80),
            ..Default::default()
        })
        .show(&ctx, |ui| {
            ui.spacing_mut().slider_width = 180.0;
            ui.spacing_mut().indent = 20.0; // these don't work currently
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 2.0); // these don't work currently

            ui.separator();
            ui.label("Tune parameters with ease");
            ui.add(egui::Slider::new(radius, 10.0..=100.0).text("Radius"));
            ui.add(toggle(custom_switch_bool));
            //ui.add(custom_slider(custom_slider_value));
            ui.add(mb_slider::MbSlider::new(&mut model.lfo1, 0.0..=1.0).text("Lfo 1"));
            ui.add(mb_slider::MbSlider::new(&mut model.lfo2, 0.0..=1.0).text("Lfo 2"));
            ui.separator();
            ui.add(mb_slider::MbSlider::new(&mut model.lfo3, 0.0..=1.0).text("Lfo 3"));
            ui.add(mb_slider::MbSlider::new(&mut model.lfo4, 0.0..=1.0).text("Lfo 4"));
            ui.add(mb_slider::MbSlider::new(&mut model.lfo5, 0.0..=1.0).text("Lfo 5"));
            nannou_egui::edit_color(ui, color);
        });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    frame.clear(BLACK);

    draw.ellipse()
        .x_y(100.0, 100.0)
        .radius(model.radius)
        .color(model.color);

    draw.to_frame(app, &frame).unwrap();

    // Do this as the last operation on your frame.
    model.egui.draw_to_frame(&frame);
}
