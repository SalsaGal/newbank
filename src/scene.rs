use std::collections::HashMap;

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
		panic!("Too many objects in Scene");
	}

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
