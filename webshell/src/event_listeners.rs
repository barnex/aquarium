use crate::*;
use web_sys::{KeyboardEvent, MouseEvent};

#[derive(Debug)]
pub(crate) enum InputEvent {
    KeyDown(KeyboardEvent),
    KeyUp(KeyboardEvent),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    //MouseMove(MouseEvent),
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
            MouseDown(event) => match event.button() {
                0 => inputs.record_press(Button::MOUSE1),
                2 => inputs.record_press(Button::MOUSE2),
                _ => (),
            },
            MouseUp(event) => match event.button() {
                0 => inputs.record_release(Button::MOUSE1),
                2 => inputs.record_release(Button::MOUSE2),
                _ => (),
            },
        }
    }
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

#[derive(Default, Debug)]
pub(crate) struct MouseEvents {
    pub left_down: Cell<bool>,
    pub right_down: Cell<bool>,
    pub pos: Cell<vec2i>,
}

pub(crate) fn listen_mouse(canvas: &HtmlCanvasElement, events: Shared<VecDeque<InputEvent>>) {
    let events_clone = Rc::clone(&events);
    let mousedown = Closure::wrap(Box::new(move |event: MouseEvent| {
        events_clone.borrow_mut().push_back(InputEvent::MouseDown(event));
    }) as Box<dyn FnMut(_)>);
    
    
    let events_clone = Rc::clone(&events);
    let mouseup = Closure::wrap(Box::new(move |event: MouseEvent| {
        events_clone.borrow_mut().push_back(InputEvent::MouseUp(event));
    }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mousedown", mousedown.as_ref().unchecked_ref()).unwrap();
    mousedown.forget();

    canvas.add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref()).unwrap();
    mouseup.forget();

    //canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
}
