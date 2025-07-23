use std::pin::Pin;

use crate::*;

pub struct Res {
    pub kitten_Todo_remove: ImageBitmap,
    cache: HashMap<Sprite, ImageBitmap>,
    pending: HashMap<Sprite, Pin<Box<dyn Future<Output = JsResult<ImageBitmap>>>>>,
}

impl Res {
    pub fn new(kitten_Todo_remove: ImageBitmap) -> Self {
        Self {
            cache: HashMap::default(),
            pending: HashMap::default(),
            kitten_Todo_remove,
        }
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
