pub mod graphics;
pub mod math;
pub mod scene;

use std::time::{Duration, Instant};

use crate::graphics::GraphicsHandler;
use crate::math::UVec2;
use crate::scene::Scene;

use scene::SceneData;
use sdl2::event::Event;
use sdl2::video::WindowBuilder;

pub struct Game {
	pub scene: Scene,
	pub window_size: WindowSize,
	pub render_framelength: Duration,
	pub update_framelength: Duration,
}

impl Game {
	pub fn run(mut self) {
		let sdl = sdl2::init().unwrap();
		let sdl_video = sdl.video().unwrap();
		let mut sdl_event = sdl.event_pump().unwrap();

		let mut window = WindowBuilder::new(&sdl_video, "Newbank", 0, 0).build().unwrap();
		match self.window_size {
			WindowSize::Fullscreen => window.set_fullscreen(sdl2::video::FullscreenType::Desktop).unwrap(),
			WindowSize::Windowed(size) => window.set_size(size.x, size.y).unwrap(),
		}
		window.set_position(sdl2::video::WindowPos::Centered, sdl2::video::WindowPos::Centered);

		let mut graphics_handler = GraphicsHandler::new(window);

		let mut next_render = Instant::now();
		let mut next_update = Instant::now();

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

			let mut scene_data = SceneData {
				graphics_handler: &mut graphics_handler,
			};

			if Instant::now().checked_duration_since(next_update).is_some() {
				self.scene.update(&mut scene_data);
				next_update = Instant::now() + self.update_framelength;
			}

			if Instant::now().checked_duration_since(next_render).is_some() {
				self.scene.render(&mut scene_data);
				graphics_handler.update();
				next_render = Instant::now() + self.render_framelength;
			}
		}
	}
}

impl Default for Game {
	fn default() -> Self {
		Self {
			scene: Scene::default(),
			window_size: WindowSize::Fullscreen,
			render_framelength: Duration::from_millis(1000 / 60),
			update_framelength: Duration::from_millis(1000 / 30),
		}
	}
}

pub enum WindowSize {
	Fullscreen,
	Windowed(UVec2),
}
