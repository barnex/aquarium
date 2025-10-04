//! Resource loader + cache
use crate::*;

use futures::task::noop_waker;
use std::pin::Pin;
use std::task::{Context, Poll};
use vector::*; // NOTE: macroquad `vec2` conflict

/// Resource loader + cache.
pub struct Resources {
    /// Loaded sprites, or red square for errored.
    cache: HashMap<Sprite, mq::Texture2D>,

    /// Sprites currently loading. Makes progress on each `poll()`.
    pending: HashMap<Sprite, Pin<Box<dyn Future<Output = mq::Texture2D>>>>,

    /// Replacement sprite to show while loading (debug only).
    while_loading: mq::Texture2D,
}

impl Resources {
    pub fn new(while_loading: mq::Texture2D) -> Self {
        Self {
            cache: HashMap::default(),
            pending: HashMap::default(),
            while_loading,
        }
    }

    /// Get bitmap for sprite.
    /// Still loading => returns `None`: can't draw yet, but will succeed soon
    /// Not found => replacement image
    pub fn get(&mut self, sprite: &Sprite) -> Option<&mq::Texture2D> {
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

    /// To be called on each tick. Newly loaded sprites become available.
    pub fn poll(&mut self) {
        // borrow checker song and dance.
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

/// load sprite over HTTP, return fallback (red square) on error.
async fn load_bitmap_or_fallback(sprite: Sprite) -> mq::Texture2D {
    const TILE_SIZE: vec2u16 = vec2(24, 24); // 🪲 TODO

    let path = format!("a_strategy_game/assets/{}.png", sprite.file.as_str());
    log::trace!("load {path:?}");
    match mq::load_texture(&path).await {
        Ok(bitmap) => bitmap,
        Err(e) => {
            log::error!("load bitmap {path}: {e:?}");
            mq::Texture2D::from_image(&fallback_bitmap((255, 0, 0), TILE_SIZE))
        }
    }
}

pub(crate) fn fallback_bitmap((r, g, b): (u8, u8, u8), size: vec2u16) -> mq::Image {
    let (width, height) = size.into();
    let color = mq::Color::from_rgba(r, g, b, 255);
    mq::Image::gen_image_color(width, height, color)
}
