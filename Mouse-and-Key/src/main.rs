// rustc 0.9 (7613b15 2014-01-08 18:04:43 -0800)

// extern crate termion;

// extern crate sdl2;

// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use std::collections::HashSet;
// use std::time::Duration;


//  mouse input showing location
// pub fn main() -> Result<(), String> {
//     let sdl_context = sdl2::init()?;
//     let video_subsystem = sdl_context.video()?;

//     let _window = video_subsystem.window("Mouse", 800, 600)
//         .position_centered()
//         .build()
//         .map_err(|e| e.to_string())?;

//     let mut events = sdl_context.event_pump()?;

//     let mut prev_buttons = HashSet::new();

//     'running: loop {
//         for event in events.poll_iter() {
//             match event {
//                 Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
//                 Event::Quit { .. } => break 'running,
//                 _ => {}
//             }
//         }

//         // get a mouse state
//         let state = events.mouse_state();

//         // Create a set of pressed Keys.
//         let buttons = state.pressed_mouse_buttons().collect();

//         // Get the difference between the new and old sets.
//         let new_buttons = &buttons - &prev_buttons;
//         let old_buttons = &prev_buttons - &buttons;

//         if !new_buttons.is_empty() || !old_buttons.is_empty() {
//             println!("X = {:?}, Y = {:?} : {:?} -> {:?}", state.x(), state.y(),  new_buttons, old_buttons);
//         }

//         prev_buttons = buttons;

//         std::thread::sleep(Duration::from_millis(100));
//     }

//     Ok(())
// }


//  keyboard input
// pub fn main() -> Result<(), String> {
//     let sdl_context = sdl2::init()?;
//     let video_subsystem = sdl_context.video()?;

//     let _window = video_subsystem.window("Keyboard", 800, 600)
//         .position_centered()
//         .build()
//         .map_err(|e| e.to_string())?;

//     let mut events = sdl_context.event_pump()?;

//     let mut prev_keys = HashSet::new();

//     'running: loop {
//         for event in events.poll_iter() {
//             if let Event::Quit {..} = event {
//                 break 'running;
//             };
//         }

//         // Create a set of pressed Keys.
//         let keys = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

//         // Get the difference between the new and old sets.
//         let new_keys = &keys - &prev_keys;
//         let old_keys = &prev_keys - &keys;

//         if !new_keys.is_empty() || !old_keys.is_empty() {
//             println!("new_keys: {:?}\told_keys:{:?}", new_keys, old_keys);
//         }

//         prev_keys = keys;

//         std::thread::sleep(Duration::from_millis(100));
//     }

//     Ok(())
// }

use std::panic;

fn main() {
	panic::set_hook(Box::new(|_| {
	    println!("Custom panic hook");
	}));

	panic!("Normal panic");
}