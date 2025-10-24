//! Install global keyboard/mouse event listeners that push events to a queue
//! (to be consumed on each tick).
use crate::*;

/// Listen for keyup/keydown events, push them to `VecDeque` for later later consumption.
pub(crate) fn listen_keys(events: Shared<VecDeque<InputEvent>>) {
    let events_clone = events.clone();
    let keydown_closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
        // mapping to lowercase for consistency with macroquad.
        let key = event.key().to_ascii_lowercase();
        if let Ok(key) = Str16::from_str(&key) {
            events_clone.borrow_mut().push_back(InputEvent::Key { button: Button(key), direction: KeyDir::Down });
        }
        if let Some(chr) = event_to_chr(event) {
            events_clone.borrow_mut().push_back(InputEvent::InputCharacter(chr));
        }
    });

    let events_clone = events.clone();
    let keyup_closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
        if let Ok(key) = Str16::from_str(&event.key()) {
            events_clone.borrow_mut().push_back(InputEvent::Key { button: Button(key), direction: KeyDir::Up });
        }
    });

    let window = web_sys::window().unwrap();

    window.add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref()).unwrap();
    window.add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref()).unwrap();

    // Keep the closures alive
    keydown_closure.forget();
    keyup_closure.forget();
}

fn event_to_chr(event: KeyboardEvent) -> Option<char> {
    // ⚠️ This is an approximate hack but otherwise extremely tricky in JS.
    // Would require shenanigans like a hidden input element that steals focus from canvas.
    log::info!("key {}", event.key());
    match event.key().as_str() {
        chr if chr.len() == 1 => chr.chars().next(),
        "Backspace" => Some('\u{0008}'),
        "Enter" => Some('\u{000D}'),
        "Escape" => Some('\u{001B}'),
        _ => None,
    }
}

/// Listen for mouse events on a canvas, push them to `VecDeque` for later later consumption.
pub(crate) fn listen_mouse(canvas: &HtmlCanvasElement, events: Shared<VecDeque<InputEvent>>) {
    let events_clone = Rc::clone(&events);
    let mousedown = Closure::wrap(Box::new(move |event: MouseEvent| {
        match event.button() {
            0 => events_clone.borrow_mut().push_back(InputEvent::Key { button: K_MOUSE1, direction: KeyDir::Down }),
            2 => events_clone.borrow_mut().push_back(InputEvent::Key { button: K_MOUSE2, direction: KeyDir::Down }),
            _ => (),
        };
    }) as Box<dyn FnMut(_)>);

    let events_clone = Rc::clone(&events);
    let mouseup = Closure::wrap(Box::new(move |event: MouseEvent| {
        match event.button() {
            0 => events_clone.borrow_mut().push_back(InputEvent::Key { button: K_MOUSE1, direction: KeyDir::Up }),
            2 => events_clone.borrow_mut().push_back(InputEvent::Key { button: K_MOUSE2, direction: KeyDir::Up }),
            _ => (),
        };
    }) as Box<dyn FnMut(_)>);

    let events_clone = Rc::clone(&events);
    let mousemove = Closure::wrap(Box::new(move |event: MouseEvent| {
        events_clone.borrow_mut().push_back(InputEvent::MouseMove {
            position: vec2(event.offset_x().as_(), event.offset_y().as_()),
        });
    }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback("mousedown", mousedown.as_ref().unchecked_ref()).unwrap();
    mousedown.forget();

    canvas.add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref()).unwrap();
    mouseup.forget();

    canvas.add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref()).unwrap();
    mousemove.forget();
}
