use winit::{
    event::{
        ElementState,
        KeyEvent,
    },
    keyboard::{
        KeyCode,
        PhysicalKey,
    },
};

pub fn process_inputs(event: &KeyEvent) -> bool {
    matches!(
        event,
        KeyEvent {
            state: ElementState::Pressed,
            physical_key: PhysicalKey::Code(KeyCode::Escape),
            ..
        }
    )
}