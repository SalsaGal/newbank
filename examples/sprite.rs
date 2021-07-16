use glam::UVec2;
use newbank::{Game, WindowSize};
use newbank::graphics::DrawPos;
use newbank::scene::{Object, Scene, SceneData};

fn main() {
	Game {
		window_size: WindowSize::Windowed(UVec2::new(640, 480)),
		scene: Scene::new().with_objects(vec![Box::new(Texture)]),
		..Default::default()
	}.run();
}

struct Texture;
impl Object for Texture {
	fn init(&mut self, data: &mut SceneData) {
		data.graphics_handler.load_texture("examples/test.png");
	}

	fn render(&self, data: &mut SceneData) {
		data.graphics_handler.draw_texture(0, DrawPos {
			x: 30,
			y: 30,
			angle: 45.0,
			..Default::default()
		});
		data.graphics_handler.draw_texture(0, DrawPos::at(300, 50));
	}
}
