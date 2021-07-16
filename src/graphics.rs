use std::collections::HashMap;

use sdl2::image::LoadTexture;
pub use sdl2::pixels::Color;

use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::video::Window;

pub type TextureID = usize;

pub struct DrawPos {
	pub x: i32,
	pub y: i32,
	pub width: Option<u32>,
	pub height: Option<u32>,
}

impl DrawPos {
	pub fn at(x: i32, y: i32) -> DrawPos {
		Self {
			x,
			y,
			..Default::default()
		}
	}
	
	pub fn size(mut self, width: u32, height: u32) -> DrawPos {
		self.width = Some(width);
		self.height = Some(height);
		self
	}
}

impl Default for DrawPos {
	fn default() -> Self {
		Self {
			x: 0,
			y: 0,
			width: None,
			height: None,
		}
	}
}

pub struct GraphicsHandler {
	pub canvas: WindowCanvas,
	pub texture_cache: HashMap<TextureID, Texture>,
}

impl GraphicsHandler {
	pub(crate) fn new(window: Window) -> Self {
		let canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();

		Self {
			canvas,
			texture_cache: HashMap::new(),
		}
	}

	pub(crate) fn update(&mut self) {
		self.canvas.present();
		self.canvas.set_draw_color(Color::BLACK);
		self.canvas.clear();
	}

	pub fn next_available_id(&self) -> TextureID {
		for index in 0..usize::MAX {
			if !self.texture_cache.contains_key(&index) {
				return index;
			}
		}
		panic!("Too many textures in texture cache!")
	}

	pub fn load_texture(&mut self, path: &str) -> TextureID {
		let id = self.next_available_id();
		self.texture_cache.insert(id, self.canvas.texture_creator().load_texture(path).unwrap());
		id
	}

	pub fn draw_texture(&mut self, texture: TextureID, pos: DrawPos) {
		self.canvas.copy_ex(
			self.texture_cache.get(&texture).unwrap(), 
			None, 
			Rect::new(
				pos.x,
				pos.y,
				match pos.width {
					Some(width) => width,
					None => todo!(),
				},
				match pos.height {
					Some(height) => height,
					None => todo!(),
				},
			), 
			0.0, 
			None, 
			false, 
			false
		).unwrap();
	}
}
