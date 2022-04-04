mod renderer;
fn main() {
    let mut screen = renderer::renderer::Renderer::new(10);

    screen
        .event_loop
        .run(move |event, _, control_flow| screen.window.request_redraw());
}
