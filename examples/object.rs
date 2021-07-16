use std::time::Duration;

use newbank::graphics::Color;
use newbank::{Game, WindowSize};
use newbank::scene::{Object, Scene, SceneData};
use newbank::math::{Rect, UVec2};

fn main() {
	Game {
		scene: Scene::new().with_objects(vec![Box::new(Foo), Box::new(Foo), Box::new(Bar)]),
		window_size: WindowSize::Windowed(UVec2::new(640, 480)),
		update_framelength: Duration::from_millis(1000 / 2),
		..Default::default()
	}.run();
}

struct Foo;
impl Object for Foo {
	fn update(&mut self, _: &mut SceneData) {
		println!("Update");
	}
}

struct Bar;
impl Object for Bar {
	fn render(&self, data: &mut SceneData) {
		data.graphics_handler.canvas.set_draw_color(Color::RED);
		data.graphics_handler.canvas.fill_rect(Rect::new(10, 32, 300, 250)).unwrap();
	}
}
