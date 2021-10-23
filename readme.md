# Rust Based Game Engine

### Goal

Game Engine will house components and systems that every game needs. Game Engine contains WebGL rendering with GLM functionality.

## Features (WIP)

- WebGL Renderer and GLSL Shaders
- GLM Functions and Data Structures
- Mesh management
- ECS management
- Javascript app, log, and input hooks

## sawd_glm

This project requires you also download my other project: [sawd_glm](https://github.com/WAFFO/sawd-glm/)

Update the `Cargo.toml` file with the path that best suits you.

## How to Include

Pull this repo and reference it locally through your cargo.toml

```toml
[dependencies.gameengine]
path = "../wip-rust-game-engine"
```

Then in your `lib.rs` you can reference it by the name given above

```rust
extern crate gameengine;
```

Below that create your application with the built in macro

```rust
//make application BB
create_app!(Template);

// MAIN
#[wasm_bindgen]
pub fn run(canvas_id : String) -> Result<Application, JsValue> {
    // hook panics to the console
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut engine = Engine::new(canvas_id)?;
    let game = Template::new(&mut engine);

    let app = Application::new(engine, game);

    Ok(app)
}
```

Now you can hook your application in javascript

```javascript
(async () => {
    // import game as webGL
    const webGL = await import('./wasm/game.js');

    // pass the id of the html canvas we're rendering to
    // receive game instance from webGL.run()
    const Game = webGL.run('canvas');

    // function that will call Game.tick() and then request another frame
    const renderLoop = () => {
        Game.tick();
        requestAnimationFrame(renderLoop);
    }

    // hook up your input functions
    document.addEventListener('mousedown', mouseDown, false);
    document.addEventListener('mouseup', mouseUp, false);
    document.addEventListener('mousemove', mouseMove, false);
    document.addEventListener('wheel', updateScroll, false);
    document.addEventListener('keydown', keyDown, false);
    document.addEventListener('keyup', keyUp, false);

    function mouseMove(e) { Game.js_mouse_move(e.screenX, e.screenY, e.movementX, e.movementY); }
    function mouseDown(e) { Game.js_mouse_press(e.button, e.buttons, e.screenX, e.screenY); }
    function mouseUp(e) { Game.js_mouse_release(e.button, e.buttons, e.screenX, e.screenY); }
    function updateScroll(e) { Game.js_mouse_scroll(e.deltaY); }
    function keyDown(e) { Game.js_key_down(e.keyCode); }
    function keyUp(e) { Game.js_key_up(e.keyCode); }

    // kick off endless loop
    requestAnimationFrame(renderLoop);
})();
```
