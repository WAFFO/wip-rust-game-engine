
use crate::engine::Engine;
use engine::input::{MouseButton, Mouse, KeyBoard};

pub trait Game {
    fn tick(&mut self, core: &mut Engine);

    // optional
    #[allow(unused_variables)]
    fn pre_tick(&mut self, core: &mut Engine) {}
    #[allow(unused_variables)]
    fn post_tick(&mut self, core: &mut Engine) {}
    #[allow(unused_variables)]
    fn receive_messages(&mut self, core: &mut Engine) {}

    // events
    #[allow(unused_variables)]
    fn event_mouse_click(&mut self, core: &mut Engine, button: MouseButton, mouse: Mouse) {}
    #[allow(unused_variables)]
    fn event_mouse_press(&mut self, core: &mut Engine, button: MouseButton, mouse: Mouse) {}
    #[allow(unused_variables)]
    fn event_mouse_release(&mut self, core: &mut Engine, button: MouseButton, mouse: Mouse) {}
    #[allow(unused_variables)]
    fn event_mouse_move(&mut self, core: &mut Engine, mouse: Mouse) {}
    #[allow(unused_variables)]
    fn event_mouse_scroll(&mut self, core: &mut Engine, scroll: f32, mouse: Mouse) {}
    #[allow(unused_variables)]
    fn event_key_press(&mut self, core: &mut Engine, key: usize, board: KeyBoard) {}
    #[allow(unused_variables)]
    fn event_key_release(&mut self, core: &mut Engine, key: usize, board: KeyBoard) {}
}
