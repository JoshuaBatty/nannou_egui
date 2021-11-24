use nannou_egui::egui;

pub fn slider_ui(ui: &mut egui::Ui, value: &mut f64) -> egui::Response {
    // Widget code can be broken up in four steps:
    //  1. Decide a size for the widget
    //  2. Allocate space for it
    //  3. Handle interactions with the widget (if any)
    //  4. Paint the widget

    // 1. Deciding widget size:
    // You can query the `ui` how much space is available,
    // but in this example we have a fixed size widget based on the height of a standard button:
    let desired_size = ui.spacing().interact_size.y * egui::vec2(10.0, 2.0);

    // 2. Allocating space:
    // This is where we get a region of the screen assigned.
    // We also tell the Ui to sense clicks in the allocated region.
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

    let mut on = &mut false;
    // 3. Interact: Time to check for clicks!
    if response.clicked() {
        *on = !*on;
        response.mark_changed(); // report back that the value changed
    }

    // Attach some meta-data to the response which can be used by screen readers:
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));


    // 4. Paint!
    // Make sure we need to paint:
    if ui.clip_rect().intersects(rect) {
        // We will follow the current style by asking
        // "how should something that is being interacted with be painted?".
        // This will, for instance, give us different colors when the widget is hovered or clicked.
        let visuals = ui.style().interact_selectable(&response, *on);
        // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
        let rect = rect.expand(visuals.expansion);
        let corner_radius = 0.1 * rect.height();
        
        let stroke = egui::Stroke::new(2.0, egui::Color32::BLACK);
        ui.painter()
            .rect(rect, corner_radius, egui::Color32::from_rgb(163, 163, 163), stroke);

        //
        let value = rect.left() + 70.0;
        let mut fill_rect = rect.shrink(1.0);
        let circle_radius = fill_rect.height() / 2.0 - 2.0;
        fill_rect.set_right(value + circle_radius);

        let fill_colour = egui::Color32::from_rgb(255,79,110);
        ui.painter()
            .rect_filled(fill_rect, corner_radius, fill_colour);

        // Paint the circle
        // let center = egui::pos2(value, rect.center().y);
        // ui.painter()
        //     .circle_stroke(center, circle_radius, visuals.fg_stroke);

        // Text
        ui.painter()
            .text(
                egui::pos2(rect.left() + 10.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                "Speed: 0.3",
                egui::TextStyle::Body,
                egui::Color32::BLACK,
            );
    }

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    response
}

pub fn custom_slider(value: &mut f64) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| slider_ui(ui, value)
}