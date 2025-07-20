use web_sys::{KeyboardEvent, MouseEvent};
use crate::*;

fn listen_keys() -> Rc<RefCell<HashSet<String>>> {
    let pressed_keys = Rc::new(RefCell::new(HashSet::default()));

    let keydown_keys = pressed_keys.clone();
    let keydown_closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
        keydown_keys.borrow_mut().insert(event.key());
    });

    let keyup_keys = pressed_keys.clone();
    let keyup_closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
        keyup_keys.borrow_mut().remove(&event.key());
    });

    let window = web_sys::window().unwrap();

    window.add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref()).unwrap();
    window.add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref()).unwrap();

    // Keep the closures alive
    keydown_closure.forget();
    keyup_closure.forget();
    pressed_keys
}

// #[derive(Default, Debug)]
// pub struct MouseEvents {
//     pub left_down: Cell<bool>,
//     pub right_down: Cell<bool>,
//     pub pos: Cell<vec2i>,
// }
// 
// fn listen_mouse(canvas: &HtmlCanvasElement) -> Rc<MouseEvents> {
//     let ev = Rc::new(MouseEvents::default());
//     let closure = {
//         let ev = ev.clone();
//         Closure::wrap(Box::new(move |event: MouseEvent| {
//             ev.pos.set(vec2(event.offset_x(), event.offset_y()));
//             ev.left_down.set(event.buttons() & 1 != 0);
//             ev.right_down.set(event.buttons() & 2 != 0);
//         }) as Box<dyn FnMut(_)>)
//     };
//     canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
//     canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
//     canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
//     closure.forget(); // Important: keep closure alive
// 
//     ev
// }