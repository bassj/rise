pub type Key = winit::event::VirtualKeyCode;
pub type MouseButton = winit::event::MouseButton;


pub struct InputManager {
    keyboard_state: [bool; 159],
    keys_just_pressed: [bool; 159],
    keys_just_released: [bool; 159],
    mouse_state: [bool; 3],
    mouse_just_pressed: [bool; 3],
    mouse_just_released: [bool; 3],
    mouse_x: f64,
    mouse_y: f64,
    mouse_motion_x: f64,
    mouse_motion_y: f64
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            keyboard_state: [false; 159],
            mouse_state: [false; 3],
            mouse_just_pressed: [false; 3],
            mouse_just_released: [false; 3],
            keys_just_pressed: [false; 159],
            keys_just_released: [false; 159],
            mouse_x: 0.,
            mouse_y: 0.,
            mouse_motion_x: 0.,
            mouse_motion_y: 0.
        }
    }

    pub fn update(&mut self) {
        self.keys_just_pressed = [false; 159];
        self.keys_just_released = [false; 159];

        self.mouse_motion_x = 0.;
        self.mouse_motion_y = 0.;

        self.mouse_just_released = [false; 3];
        self.mouse_just_pressed = [false; 3];
    }

    pub fn process_event(&mut self, event: &winit::event::Event<()>) {
        match event {
            winit::event::Event::DeviceEvent {
                event,
                device_id
            } => {
                self.process_device_input(event);
            },
            winit::event::Event::WindowEvent {
                event,
                window_id
            } => {
                self.process_window_input(event);
            }
            _ => {}
        }
    }
    
    fn process_device_input(&mut self, event: &winit::event::DeviceEvent) {
        match event {
            winit::event::DeviceEvent::MouseMotion {
                delta
            } => {
                self.process_mouse_motion(*delta);
            },
            _ => {}
        }
    }

    fn process_window_input(&mut self, event: &winit::event::WindowEvent) {
        match event {
            winit::event::WindowEvent::KeyboardInput {
                input,
                device_id,
                is_synthetic
            } => {
                self.process_keyboard_input(input);
            },
            winit::event::WindowEvent::CursorMoved {
                position,
                modifiers,
                device_id
            } => {
                self.process_mouse_position(position);
            },
            winit::event::WindowEvent::MouseInput {
                device_id,
                state,
                button,
                modifiers
            } => {
                self.process_mouse_button(button, state);
            }
            _ => {} 
        }
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.keyboard_state[key as usize]
    }

    pub fn is_just_pressed(&self, key: Key) -> bool {
        self.keys_just_pressed[key as usize]
    }

    pub fn is_just_released(&self, key: Key) -> bool {
        self.keys_just_released[key as usize]
    }

    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => {
                self.mouse_state[0]
            },
            MouseButton::Middle => {
                self.mouse_state[1]
            },
            MouseButton::Right => {
                self.mouse_state[2]
            },
            _ => false
        }
    }

    pub fn is_mouse_just_pressed(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => {
                self.mouse_just_pressed[0]
            },
            MouseButton::Middle => {
                self.mouse_just_pressed[1]
            },
            MouseButton::Right => {
                self.mouse_just_pressed[2]
            },
            _ => false
        }
    }

    pub fn is_mouse_just_released(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => {
                self.mouse_just_released[0]
            },
            MouseButton::Middle => {
                self.mouse_just_released[1]
            },
            MouseButton::Right => {
                self.mouse_just_released[2]
            },
            _ => false
        }
    }

    pub fn mouse_position(&self) -> (f64, f64) {
        (self.mouse_x, self.mouse_y)
    }

    pub fn mouse_motion(&self) -> (f64, f64) {
        (self.mouse_motion_x, self.mouse_motion_y)
    }

    fn process_keyboard_input(&mut self, event: &winit::event::KeyboardInput) {
        match &event.virtual_keycode {
            &Some(keycode) => {
                let key = keycode as usize;
    
                match &event.state {
                    winit::event::ElementState::Pressed => {
                        self.keyboard_state[key] = true;
                        self.keys_just_pressed[key] = true;
                    },
                    winit::event::ElementState::Released => {
                        self.keyboard_state[key] = false;
                        self.keys_just_released[key] = true;
                    }
                }
            },
            None => {}
        }
    }

    fn process_mouse_position(&mut self, position: &winit::dpi::PhysicalPosition<f64>) {
        self.mouse_x = position.x;
        self.mouse_y = position.y;
    }

    fn process_mouse_motion(&mut self, motion: (f64, f64)) {
        self.mouse_motion_x += motion.0;
        self.mouse_motion_y += motion.1;
    }

    fn process_mouse_button(&mut self, button: &MouseButton, state: &winit::event::ElementState) {
        let pressed = state == &winit::event::ElementState::Pressed;

        match button {
            MouseButton::Left => {
                self.mouse_state[0] = pressed;
                self.mouse_just_pressed[0] = pressed;
                self.mouse_just_released[0] = !pressed;
            },
            MouseButton::Middle => {
                self.mouse_state[1] = pressed;
                self.mouse_just_pressed[1] = pressed;
                self.mouse_just_released[1] = !pressed;
            },
            MouseButton::Right => {
                self.mouse_state[2] = pressed;
                self.mouse_just_pressed[2] = pressed;
                self.mouse_just_released[2] = !pressed;
            },
            _ => {}
        }
    }

}