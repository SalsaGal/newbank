use glam::UVec2;

use sdl2::event::Event;
use sdl2::video::WindowBuilder;

pub struct Game {
	pub window_size: WindowSize,
}

impl Game {
	pub fn run(self) {
		let sdl = sdl2::init().unwrap();
		let sdl_video = sdl.video().unwrap();
		let mut sdl_event = sdl.event_pump().unwrap();

		let mut window = WindowBuilder::new(&sdl_video, "Newbank", 0, 0).build().unwrap();
		match self.window_size {
			WindowSize::Fullscreen => window.set_fullscreen(sdl2::video::FullscreenType::Desktop).unwrap(),
			WindowSize::Windowed(size) => window.set_size(size.x, size.y).unwrap(),
		}

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
			window_size: WindowSize::Fullscreen,
		}
	}
}

pub enum WindowSize {
	Fullscreen,
	Windowed(UVec2),
}
