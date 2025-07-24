use std::pin::Pin;

use crate::*;

pub struct Res {
    fallback: ImageBitmap,
    cache: HashMap<Sprite, ImageBitmap>,
    pending: HashMap<Sprite, Pin<Box<dyn Future<Output = JsResult<ImageBitmap>>>>>,
}

impl Res {
    pub fn new(fallback: ImageBitmap) -> Self {
        Self {
            cache: HashMap::default(),
            pending: HashMap::default(),
            fallback,
        }
    }
	
	pub fn get(&mut self, sprite: &Sprite) -> Option<&ImageBitmap>{
		if let Some(bitmap)	 = self.cache.get(sprite){
			return Some(bitmap)
		}
		
		Some(&self.fallback) // 🪲 
	}
	
	pub fn poll() {
		
	}
}

fn box_that_fut() {
    let x = Box::pin(load_image_future("kit3.png"));
    let mut tasks = HashMap::default();
    tasks.insert(42, x);
}

fn load_image_future(path: &str) -> impl Future<Output = JsResult<ImageBitmap>> {
    load_image(path)
}
