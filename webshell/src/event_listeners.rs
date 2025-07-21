use crate::*;
use web_sys::{KeyboardEvent, MouseEvent};

pub(crate) fn record_key_events(inputs: &mut Inputs, key_events: &Rc<RefCell<VecDeque<(String, bool)>>>) {
    for (key, pressed) in key_events.borrow_mut().drain(..) {
        info!("record key {key} {pressed}");

        match (Str16::from_str(key.as_str()), pressed) {
            (Ok(key), true) => inputs.record_press(Button(key)),
            (Ok(key), false) => inputs.record_release(Button(key)),
            (Err(e), _) => log::error!("ignored key {key}: {e}"),
        }
    }
}

/// Listen for keyboard events: returns shared channel (`VecDeque`) where events are being pushed (`push_back`).
/// Consume the events via `pop_front` or `iter_mut`.
///
/// Event = (Key, pressed). e.g.
///     ("a", true)  // key A pressed
///     ("b", false) // key B released
///    
pub(crate) fn listen_keys() -> Rc<RefCell<VecDeque<(String, bool)>>> {
    let key_events = Rc::new(RefCell::new(VecDeque::new()));

    let keydown_keys = key_events.clone();
    let keydown_closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
        keydown_keys.borrow_mut().push_back((event.key(), true));
    });

    let keyup_keys = key_events.clone();
    let keyup_closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
        keyup_keys.borrow_mut().push_back((event.key(), false));
    });

    let window = web_sys::window().unwrap();

    window.add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref()).unwrap();
    window.add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref()).unwrap();

    // Keep the closures alive
    keydown_closure.forget();
    keyup_closure.forget();
    key_events
}

#[derive(Default, Debug)]
pub(crate) struct MouseEvents {
    pub left_down: Cell<bool>,
    pub right_down: Cell<bool>,
    pub pos: Cell<vec2i>,
}

pub(crate) fn listen_mouse(canvas: &HtmlCanvasElement) -> Rc<MouseEvents> {
    let ev = Rc::new(MouseEvents::default());
    let closure = {
        let ev = ev.clone();
        Closure::wrap(Box::new(move |event: MouseEvent| {
            ev.pos.set(vec2(event.offset_x(), event.offset_y()));
            ev.left_down.set(event.buttons() & 1 != 0);
            ev.right_down.set(event.buttons() & 2 != 0);
        }) as Box<dyn FnMut(_)>)
    };
    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
    canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget(); // Important: keep closure alive

    ev
}
