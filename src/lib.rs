use sdl2::event::Event;
use sdl2::video::WindowBuilder;

pub struct Game {
}

impl Game {
	pub fn run(self) {
		let sdl = sdl2::init().unwrap();
		let sdl_video = sdl.video().unwrap();
		let mut sdl_event = sdl.event_pump().unwrap();

		let _window = WindowBuilder::new(&sdl_video, "Newbank", 640, 480).build().unwrap();

		let mut running = true;
		while running {
			for event in sdl_event.poll_iter() {
				match event {
					Event::Quit { .. } => {
						running = false;
					},
					_ => {},
				}
			}
		}
	}
}

impl Default for Game {
	fn default() -> Self {
		Self {
		}
	}
}
