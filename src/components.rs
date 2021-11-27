use std::collections::HashSet;

//https://github.com/bevyengine/bevy/blob/main/examples/game/alien_cake_addict.rs

struct GameConfig {
    title: String,
    width: u32,
    height: u32,
    startingScore: u8, //20,
    crabSpeed: f32,    // 2.2,
    ballSpeed: f32,    // ??
}

enum CrabId {
    Orange,
    Blue,
    Red,
    Purple,
}

enum GameScreen {
    Won,
    GameOver,
    NewGame,
    Playing,
}

struct Game {
    scores: HashSet<CrabId, f32>,
    screen: GameScreen,
}

// TODO: Use this in place of positive and negative numbers.
enum CrabMovementDirection {
    Idle,
    Left,
    Right,
}

struct Crab {
    id: CrabId,
    // color: Color,
    direction: CrabMovementDirection,
    /* TODO: Maybe store a Vec2 'mask' for handling ball collision axis in a
     * generic way? TODO: How to handle zero score shrinking effect? */
}

struct Score {
    crab_id: CrabId,
}

struct Ball {
    active: bool, /* Whether it is still updating or frozen in place while
                   * fading? */
    opacity: f32,
}

enum Collider {
    Circle {
        radius: f32,
    },
    Box {
        /* direction: Vec2, */ width: f32,
        height: f32,
    },
}

struct Pole {
    crab_id: CrabId,
    is_active: bool,
}

// TODO: For all the balls AND crabs?
struct Velocity {
    angle: f32, // radians
    speed: f32,
}

enum Pilot {
    Player,
    Ai,
}

// TODO: .single_mut()
struct Water {
    scroll: f32,
}

// --Systems--
// One update function for all crabs?
// * load_initial_scene()
// * load_game_over_scene()
// * load_new_game_scene() Delay before spawning balls?
// * update_scores() Crab whose score hits zero needs to have direction set to
//   Idle and speed set to zero immediately!
// * player_input()
// * enemy_ai() May need to work on a fixed timestep.
// * move_balls() // Handles ball resets as well?
// * sway_camera()
// * animate_water()

// -- General --
// Instead of mirroring, have reflections be child entities and just move one
// entity?      What about if we want to add animation in the future?
