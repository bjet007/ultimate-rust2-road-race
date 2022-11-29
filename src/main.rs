use rusty_engine::prelude::*;
use rand::prelude::*;

const PLAYER_LABEL: &'static str = "player1";
const HEALTH_LABEL: &'static str = "health_message";
const ROAD_LINE_LABEL_PREFIX: &'static str = "roadline_";
const OBSTACLE_LABEL_PREFIX: &'static str = "obstacle_";
const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;
// Define a struct to hold custom data for your game (it can be a lot more complicated than this one!)
struct GameState {
    health_amount: u8,
    lost: bool,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            health_amount: 5,
            lost: false,
        }
    }
}



fn main() {
    // Create a game
    let mut game = Game::new();
    // Set up your game. `Game` exposes all of the methods and fields of `Engine`.
    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    let player1 = game.add_sprite(PLAYER_LABEL, SpritePreset::RacingCarBlue);
    player1.translation.x = -500.0;
    player1.layer = 10.0;
    player1.collision = true;

    // Create the road lines
    for i in 0..10 {
        let roadline = game.add_sprite(format!("{}{}", ROAD_LINE_LABEL_PREFIX, i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }

    let obstacle_presets = vec![SpritePreset::RacingBarrelBlue, SpritePreset::RacingBarrelRed, SpritePreset::RacingConeStraight];
    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("{}{}",OBSTACLE_LABEL_PREFIX, i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    let health_message = game.add_text("health_message", "Health: 5");
    health_message.translation = Vec2::new(550.0, 320.0);

    // Add one or more functions with logic for your game. When the game is run, the logic
    // functions will run in the order they were added.
    game.add_logic(game_logic);
    // Run the game, with an initial state
    let initial_game_state = GameState::default();
    game.run(initial_game_state);
}


// Your game logic functions can be named anything, but the first parameter is always a
// `&mut Engine`, and the second parameter is a mutable reference to your custom game
// state struct (`&mut GameState` in this case).
//
// This function will be run once each frame.
fn game_logic(engine: &mut Engine, game_state: &mut GameState) {

    // Don't run any more game logic if the game has ended
    if game_state.lost {
        return;
    }

    let mut direction: f32 = 0.0;

    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    // Move road objects
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with(ROAD_LINE_LABEL_PREFIX) {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
        }

        // inside the same loop in `game_logic` that moves the road lines
        if sprite.label.starts_with(OBSTACLE_LABEL_PREFIX) {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }



    // The `Engine` contains all sorts of built-in goodies.
    // Get access to the player sprite...
    let player = engine.sprites.get_mut(PLAYER_LABEL).unwrap();

    player.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player.rotation = direction * 0.15;
    if player.translation.y < -360.0 || player.translation.y > 360.0 {
        game_state.health_amount = 0;
    }

    let health_message = engine.texts.get_mut(HEALTH_LABEL).unwrap();
    for event in engine.collision_events.drain(..) {
        // We don't care if obstacles collide with each other or collisions end
        if !event.pair.either_contains(PLAYER_LABEL) || event.state.is_end() {
            continue;
        }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!("Health: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
        }
    }

    if game_state.health_amount == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game over", "Game Over");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}