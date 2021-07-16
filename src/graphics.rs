pub use sdl2::pixels::Color;

use sdl2::render::WindowCanvas;
use sdl2::video::Window;

pub struct GraphicsHandler {
	pub canvas: WindowCanvas,
}

impl GraphicsHandler {
	pub(crate) fn new(window: Window) -> Self {
		let canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();

		Self {
			canvas,
		}
	}

	pub(crate) fn update(&mut self) {
		self.canvas.present();
		self.canvas.set_draw_color(Color::BLACK);
		self.canvas.clear();
	}
}
