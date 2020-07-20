use graphics::Renderer;

pub struct Window {
    pub width: f32,
    pub height: f32,
    pub resizable: bool,
    pub title: &'static str,
}

pub struct GameEnvironment {
    pub window: Window,
}

pub struct Game {}

impl Game {
    pub fn new(env: GameEnvironment, renderer: &Renderer) -> Game {
        Game {}
    }

    pub fn update(&mut self, delta: f32) {
        unimplemented!();
    }

    pub fn render(&mut self, renderer: &Renderer) {
        renderer.clear();
        
        //unimplemented!();
    }
}