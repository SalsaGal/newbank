use newbank::{Game, WindowSize};
use newbank::math::UVec2;

fn main() {
	Game {
		window_size: WindowSize::Windowed(UVec2::new(640, 480)),
		..Default::default()
	}.run();
}
