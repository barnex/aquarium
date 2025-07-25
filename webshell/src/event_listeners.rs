//! Install global keyboard/mouse event listeners that push events to a queue
//! (to be consumed on each tick).
use crate::*;

/// Keyboard/Mouse event.
/// Bridges the mismatch between JavaScript events and winit-like events used by the game.
#[derive(Debug)]
pub(crate) enum InputEvent {
    KeyDown(KeyboardEvent),
    KeyUp(KeyboardEvent),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseMove(MouseEvent),
}

/// Listen for keyup/keydown events, push them to `VecDeque` for later later consumption.
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

/// Listen for mouse events on a canvas, push them to `VecDeque` for later later consumption.
pub(crate) fn listen_mouse(canvas: &HtmlCanvasElement, events: Shared<VecDeque<InputEvent>>) {
    let events_clone = Rc::clone(&events);
    let mousedown = Closure::wrap(Box::new(move |event: MouseEvent| {
        events_clone.borrow_mut().push_back(InputEvent::MouseDown(event));
    }) as Box<dyn FnMut(_)>);

    let events_clone = Rc::clone(&events);
    let mouseup = Closure::wrap(Box::new(move |event: MouseEvent| {
        events_clone.borrow_mut().push_back(InputEvent::MouseUp(event));
    }) as Box<dyn FnMut(_)>);

    let events_clone = Rc::clone(&events);
    let mousemove = Closure::wrap(Box::new(move |event: MouseEvent| {
        events_clone.borrow_mut().push_back(InputEvent::MouseMove(event));
    }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mousedown", mousedown.as_ref().unchecked_ref()).unwrap();
    mousedown.forget();

    canvas.add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref()).unwrap();
    mouseup.forget();

    canvas.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref()).unwrap();
    mousemove.forget();
}
