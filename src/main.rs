use ggez::{
    timer,
    conf, 
    graphics,
    // input, 
    event, 
    GameError, 
    GameResult, 
    Context, 
    ContextBuilder
};
struct State {
    dt: std::time::Duration,
    fps: ggez::graphics::Text,
    text_scale: f32
}

impl ggez::event::EventHandler<GameError> for State {
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: event::MouseButton, _x: f32, _y: f32) {
        println!("Mouse X: {}, Mouse Y {}", _x, _y);
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: event::KeyCode, _keymods: event::KeyMods, _repeat: bool) {
        println!("Pressed: {:?}", keycode);
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        let fps = 1000000000 / self.dt.as_nanos();
        self.fps = graphics::Text::new(fps.to_string());
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let (width, height) = graphics::drawable_size(ctx);

        graphics::queue_text(ctx, &self.fps, cgmath::Point2::new(0.0, 0.0), None);

        let mut rustopia_text = graphics::Text::new("Rustopia");
        let rustopia_text_width = &rustopia_text.width(ctx);

        graphics::queue_text(
            ctx, 
            &rustopia_text.set_font(graphics::Font::default(), graphics::PxScale::from(self.text_scale)), 
            cgmath::Point2::new(width / 2. - rustopia_text_width * 4., height / 2.5), 
            Some(graphics::Color::new(239., 74., 0., 1.))
        );
        graphics::draw_queued_text(
            ctx, 
            graphics::DrawParam::default(),
            None,
            graphics::FilterMode::Linear
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let state = State {
        dt: std::time::Duration::new(0, 0),
        fps: graphics::Text::new(""),
        text_scale: 128.
    };
    
    let window_setup = conf::WindowSetup {
        title: "Rustopia".to_string(),
        samples: conf::NumSamples::One,
        vsync: false,
        icon: "".to_owned(),
        srgb: true,
    };

    let mut window_mode = ggez::conf::WindowMode::default();
    window_mode.width = 1024.;
    window_mode.height = 768.;

    let c = conf::Conf {
        window_mode: window_mode,
        window_setup: window_setup,
        backend: conf::Backend::default(),
        modules: conf::ModuleConf::default()
    };
    let (ctx, event_loop) = ContextBuilder::new("rustopia", "gopherz")
        .default_conf(c)
        .build()
        .unwrap();
    
    event::run(ctx, event_loop, state);
}