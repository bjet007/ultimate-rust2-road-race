use rusty_engine::prelude::*;

// Define a struct to hold custom data for your game (it can be a lot more complicated than this one!)
struct GameState {
    health: i32,
}

fn main() {
    // Create a game
    let mut game = Game::new();
    // Set up your game. `Game` exposes all of the methods and fields of `Engine`.
    let sprite = game.add_sprite("player", SpritePreset::RacingCarBlue);
    sprite.scale = 2.0;
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 1.0);
    // Add one or more functions with logic for your game. When the game is run, the logic
    // functions will run in the order they were added.
    game.add_logic(game_logic);
    // Run the game, with an initial state
    let initial_game_state = GameState { health: 100 };
    game.run(initial_game_state);
}

// Your game logic functions can be named anything, but the first parameter is always a
// `&mut Engine`, and the second parameter is a mutable reference to your custom game
// state struct (`&mut GameState` in this case).
//
// This function will be run once each frame.
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // The `Engine` contains all sorts of built-in goodies.
    // Get access to the player sprite...
    let player = engine.sprites.get_mut("player").unwrap();
    // Rotate the player...
    player.rotation += std::f32::consts::PI * engine.delta_f32;
    // Damage the player if it is out of bounds...
    if player.translation.x > 100.0 {
        game_state.health -= 1;
    }
}