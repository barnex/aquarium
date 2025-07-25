//! Resource loader + cache

use crate::*;
use futures::task::noop_waker;
use std::task::{Context, Poll};

/// Resource loader + cache.
pub struct Resources {
    /// Loaded sprites, or red square for errored.
    cache: HashMap<Sprite, ImageBitmap>,

    /// Sprites currently loading. Makes progress on each `poll()`.
    pending: HashMap<Sprite, Pin<Box<dyn Future<Output = ImageBitmap>>>>,

    /// Replacement sprite to show while loading (debug only).
    while_loading: ImageBitmap,
}

impl Resources {
    pub fn new(while_loading: ImageBitmap) -> Self {
        Self {
            cache: HashMap::default(),
            pending: HashMap::default(),
            while_loading,
        }
    }

    /// Get bitmap for sprite.
    /// Still loading => returns `None`: can't draw yet, but will succeed soon
    /// Not found => replacement image
    pub fn get(&mut self, sprite: &Sprite) -> Option<&ImageBitmap> {
        if let Some(bitmap) = self.cache.get(sprite) {
            return Some(bitmap);
        }

        if self.pending.contains_key(sprite) {
            return None; // still loading, try again next frame
        }

        self.pending.insert(*sprite, Box::pin(load_bitmap_or_fallback(*sprite)));

        #[cfg(debug_assertions)]
        {
            Some(&self.while_loading)
        }
        #[cfg(not(debug_assertions))]
        {
            None
        }
    }

    pub fn poll(&mut self) {
        let mut ready = Vec::new();

        for (sprite, fut) in self.pending.iter_mut() {
            let waker = noop_waker();
            let mut cx = Context::from_waker(&waker);

            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(val) => ready.push((*sprite, val)),
                Poll::Pending => (),
            }
        }

        for (sprite, val) in ready {
            self.pending.remove(&sprite);
            self.cache.insert(sprite, val);
        }
    }
}

async fn load_bitmap_or_fallback(sprite: Sprite) -> ImageBitmap {
    let path = format!("assets/{}.png", sprite.file.as_str());
    match load_bitmap(&path).await {
        Ok(bitmap) => bitmap,
        Err(e) => {
            log::error!("load bitmap {path}: {e:?}");
            fallback_bitmap(255, 0, 0).await.unwrap()
        }
    }
}

fn box_that_fut() {
    let x = Box::pin(load_image_future("kit3.png"));
    let mut tasks = HashMap::default();
    tasks.insert(42, x);
}

fn load_image_future(path: &str) -> impl Future<Output = JsResult<ImageBitmap>> {
    load_bitmap(path)
}
