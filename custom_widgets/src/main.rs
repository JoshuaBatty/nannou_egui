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
    texture: wgpu::Texture,
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
    let device = window.device();
    let queue = window.queue();
    let usage = wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING;

    // Load the image from disk and upload it to a GPU texture.
    let path = std::path::Path::new("assets/images/nannou.png");
    let texture = wgpu::Texture::load_from_path(device, queue, usage, path).unwrap();

    Model {
        egui: Egui::from_window(&window),
        radius: 40.0,
        color: hsv(10.0, 0.5, 1.0),
        custom_switch_bool: false,
        lfo1: 0.5,
        lfo2: 0.3,
        lfo3: 0.6,
        lfo4: 0.12,
        lfo5: 0.4,
        texture,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let Model {
        ref mut egui,
        ref mut radius,
        ref mut color,
        ref mut custom_switch_bool,
        ..
    } = *model;

    let window = app.main_window();
    let device = window.device();
    let texture_id = egui.renderer.borrow_mut().render_pass.egui_texture_from_wgpu_texture(
        &device,
        &model.texture,
        nannou::wgpu::FilterMode::Nearest
    );

    println!("texture_id: {:?}", texture_id);

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

    let mut style = egui::Style::default();

    style.visuals.widgets.inactive.corner_radius = 2.0;
    style.visuals.widgets.hovered.corner_radius = 2.0;
    style.visuals.widgets.active.corner_radius = 2.0;

    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_gray(163);
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_gray(163);
    style.visuals.widgets.active.bg_fill = egui::Color32::from_gray(183);

    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(2.0, egui::Color32::BLACK);
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(2.0, egui::Color32::BLACK);
    style.visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0, egui::Color32::BLACK);

    style.visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(255,79,110);
    style.visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgb(255,79,110);
    style.visuals.widgets.active.fg_stroke.color = egui::Color32::from_rgb(255,79,100);

    style.spacing.slider_width = 180.0;
    style.spacing.item_spacing = egui::vec2(0.0, 4.0);

    let [w, h] = model.texture.size();
    let image_size = egui::Vec2::new(w as f32 * 0.2 ,h as f32 * 0.2 );

    egui::Area::new("my_area").fixed_pos([8.0, 8.0]).show(&ctx, |ui| {
        ctx.set_style(style);
        ui.colored_label(egui::Color32::WHITE, "PARAMETERS");
        let slider_height = 36.0;
        ui.add(mb_slider::MbSlider::new(&mut model.lfo1, 0.0..=1.0).height(slider_height).text("Lfo 1"));
        ui.add(mb_slider::MbSlider::new(&mut model.lfo2, 0.0..=1.0).height(slider_height).text("Lfo 2"));
        ui.add(mb_slider::MbSlider::new(&mut model.lfo3, 0.0..=1.0).height(slider_height).text("Lfo 3"));
        ui.add(mb_slider::MbSlider::new(&mut model.lfo4, 0.0..=1.0).height(slider_height).text("Lfo 4"));
        ui.add(mb_slider::MbSlider::new(&mut model.lfo5, 0.0..=1.0).height(slider_height).text("Lfo 5"));
        ui.image(texture_id, image_size); 
    });
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    frame.clear(GRAY);

    draw.ellipse()
        .x_y(100.0, (app.time * (model.lfo1 as f32 * 10.0)).sin() * (model.lfo2 as f32 * 200.0))
        .radius(model.lfo2 as f32 * 50.0)
        .rgb(model.lfo3 as f32, model.lfo4 as f32, model.lfo5 as f32);

    draw.to_frame(app, &frame).unwrap();

    // Do this as the last operation on your frame.
    model.egui.draw_to_frame(&frame);
}
