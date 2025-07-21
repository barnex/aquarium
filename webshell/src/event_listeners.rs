use crate::*;
use web_sys::{KeyboardEvent, MouseEvent};

#[derive(Debug)]
pub(crate) enum InputEvent {
    KeyDown(KeyboardEvent),
    KeyUp(KeyboardEvent),
}

pub(crate) fn record_input_events(inputs: &mut Inputs, events: &Shared<VecDeque<InputEvent>>) {
    for event in events.borrow_mut().drain(..) {
        info!("record input event {event:?}");

        use InputEvent::*;
        match event {
            KeyDown(event) => {
                if let Ok(key) = Str16::from_str(&event.key()) {
                    inputs.record_press(Button(key))
                }
            }
            KeyUp(event) => {
                if let Ok(key) = Str16::from_str(&event.key()) {
                    inputs.record_release(Button(key))
                }
            }
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
pub(crate) fn listen_keys(events: Shared<VecDeque<InputEvent>>) {
    let keydown_keys = events.clone();
    let keydown_closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
        keydown_keys.borrow_mut().push_back(InputEvent::KeyDown(event));
    });

    let keyup_keys = events.clone();
    let keyup_closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
        keyup_keys.borrow_mut().push_back(InputEvent::KeyUp(event));
    });

    let window = web_sys::window().unwrap();

    window.add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref()).unwrap();
    window.add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref()).unwrap();

    // Keep the closures alive
    keydown_closure.forget();
    keyup_closure.forget();
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
