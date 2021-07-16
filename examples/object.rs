use newbank::{Game, WindowSize};
use newbank::scene::{Object, Scene};
use newbank::math::UVec2;

fn main() {
	Game {
		scene: Scene::new().with_objects(vec![Box::new(Foo), Box::new(Foo), Box::new(Bar)]),
		window_size: WindowSize::Windowed(UVec2::new(640, 480)),
		..Default::default()
	}.run();
}

struct Foo;
impl Object for Foo {
	fn update(&mut self) {
		println!("Foo");
	}
}

struct Bar;
impl Object for Bar {
	fn update(&mut self) {
		println!("Bar");
	}
}
