use std::collections::HashMap;

pub type ObjectID = usize;

pub struct Scene {
	objects: HashMap<ObjectID, Box<dyn Object>>,
}

impl Scene {
	pub(crate) fn update(&mut self) {
		for (_, object) in self.objects.iter_mut() {
			object.update();
		}
	}

	pub(crate) fn render(&self) {
		for (_, object) in self.objects.iter() {
			object.render();
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

pub trait Object {
	fn update(&mut self) {}
	fn render(&self) {}
}

pub struct EmptyObject;
impl Object for EmptyObject {}
