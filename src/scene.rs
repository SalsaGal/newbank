use std::collections::HashMap;

use crate::graphics::GraphicsHandler;

pub type ObjectID = usize;

pub struct Scene {
	objects: HashMap<ObjectID, Box<dyn Object>>,
}

impl Scene {
	pub fn new() -> Self {
		Self {
			objects: HashMap::new(),
		}
	}

	pub fn with_objects(mut self, objects: Vec<Box<dyn Object>>) -> Self {
		for object in objects.into_iter() {
			self.spawn_object(object);
		}
		self
	}

	pub fn spawn_object(&mut self, object: Box<dyn Object>) -> ObjectID {
		let id = self.next_available_id();
		self.objects.insert(id, object);
		id
	}

	pub fn next_available_id(&self) -> ObjectID {
		for index in 0..usize::MAX {
			if !self.objects.contains_key(&index) {
				return index;
			}
		}
		panic!("Too many objects in scene!");
	}

	pub(crate) fn update(&mut self, scene_data: &mut SceneData) {
		for (_, object) in self.objects.iter_mut() {
			object.update(scene_data);
		}
	}

	pub(crate) fn render(&self, scene_data: &mut SceneData) {
		for (_, object) in self.objects.iter() {
			object.render(scene_data);
		}
	}
}

impl Default for Scene {
	fn default() -> Self {
		Self {
			objects: HashMap::new(),
		}
	}
}

pub struct SceneData<'data> {
	pub graphics_handler: &'data mut GraphicsHandler,
}

pub trait Object {
	fn init(&mut self, _: &mut SceneData) {}
	fn update(&mut self, _: &mut SceneData) {}
	fn render(&self, _: &mut SceneData) {}
}

pub struct EmptyObject;
impl Object for EmptyObject {}
