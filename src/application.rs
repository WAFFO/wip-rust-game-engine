//use wasm_bindgen;
//use wasm_bindgen::JsValue;
//
//use engine::Engine;
//use game::Game;


#[macro_export]
macro_rules! create_app {
    ( $x:ty ) => {
        #[wasm_bindgen]
        pub struct Application {
            engine: Engine,
            game: $x,
        }

        #[wasm_bindgen]
        impl Application {

            #[wasm_bindgen]
            pub fn new(engine: Engine, game: $x) -> Application {
                Application {
                    engine,
                    game,
                }
            }

            #[wasm_bindgen]
            pub fn tick(&mut self) -> Result<(), JsValue> {
                self.engine.tick();
                self.game.tick(&mut self.engine);
                self.engine.render()
            }

            #[wasm_bindgen]
            pub fn js_mouse_click(&mut self, button: u32, buttons: u32, x: f32, y: f32) {
                let (button, mouse) = self.engine.js_mouse_click(button, buttons, x, y);
                self.game.event_mouse_click(&mut self.engine, button, mouse)
            }

            #[wasm_bindgen]
            pub fn js_mouse_press(&mut self, button: u32, buttons: u32, x: f32, y: f32) {
                let (button, mouse) = self.engine.js_mouse_press(button, buttons, x, y);
                self.game.event_mouse_press(&mut self.engine, button, mouse)
            }

            #[wasm_bindgen]
            pub fn js_mouse_release(&mut self, button: u32, buttons: u32, x: f32, y: f32) {
                let (button, mouse) = self.engine.js_mouse_release(button, buttons, x, y);
                self.game.event_mouse_release(&mut self.engine, button, mouse)
            }

            #[wasm_bindgen]
            pub fn js_mouse_move(&mut self, x: f32, y: f32, move_x: f32, move_y: f32) {
                let mouse = self.engine.js_mouse_move(x, y, move_x, move_y);
                self.game.event_mouse_move(&mut self.engine, mouse)
            }

            #[wasm_bindgen]
            pub fn js_mouse_scroll(&mut self, scroll: f32) {
                let mouse = self.engine.js_mouse_scroll(scroll);
                self.game.event_mouse_scroll(&mut self.engine, scroll, mouse)
            }

            #[wasm_bindgen]
            pub fn js_key_down(&mut self, key: usize) {
                let key_board = self.engine.js_key_down(key);
                self.game.event_key_press(&mut self.engine, key, key_board)
            }

            #[wasm_bindgen]
            pub fn js_key_up(&mut self, key: usize) {
                let key_board = self.engine.js_key_up(key);
                self.game.event_key_release(&mut self.engine, key, key_board)
            }
        }
    }
}
