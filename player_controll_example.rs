use godot::classes::{
    AnimatedSprite2D, AnimationPlayer, Area2D, CharacterBody2D, ICharacterBody2D,
};
use godot::prelude::*;

/// Character states used by the finite state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Idle,
    Run,
    Fall,
    Attack,
}

/// Handles sprite and animation playback.
struct AnimationController {
    player: Option<Gd<AnimationPlayer>>,
    sprite: Option<Gd<AnimatedSprite2D>>,
    attack_area: Option<Gd<Area2D>>,

    current_sprite: String,
    current_animation: String,
    animation_finished: bool,
}

impl AnimationController {
    fn new() -> Self {
        Self {
            player: None,
            sprite: None,
            attack_area: None,
            current_sprite: String::new(),
            current_animation: String::new(),
            animation_finished: true,
        }
    }

    fn play_sprite(&mut self, name: &str) {
        if let Some(sprite) = &mut self.sprite {
            if self.current_sprite != name {
                sprite.play_ex().name(name).done();
                self.current_sprite = name.to_string();
            }
        }
    }

    fn play_animation(&mut self, name: &str) {
        if !self.animation_finished || self.current_animation == name {
            return;
        }

        if let Some(player) = &mut self.player {
            self.animation_finished = false;
            player.play_ex().name(name).done();
            self.current_animation = name.to_string();
        }
    }

    fn is_flipped(&self) -> bool {
        self.sprite
            .as_ref()
            .map(|s| s.is_flipped_h())
            .unwrap_or(false)
    }
}

/// Runtime movement data.
struct MovementController {
    velocity_x: f32,
    velocity_y: f32,

    direction: f32,

    speed: f32,
    gravity: f32,
    jump_force: f32,

    attack_requested: bool,
}

impl MovementController {
    fn new() -> Self {
        Self {
            velocity_x: 0.0,
            velocity_y: 0.0,
            direction: 0.0,
            speed: 300.0,
            gravity: 1200.0,
            jump_force: -400.0,
            attack_requested: false,
        }
    }

    fn velocity(&self) -> Vector2 {
        Vector2::new(self.velocity_x, self.velocity_y)
    }
}

#[derive(GodotClass)]
#[class(base = CharacterBody2D)]
pub struct Player {
    state: State,
    animation: AnimationController,
    movement: MovementController,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            movement: MovementController::new(),
            state: State::Idle,
            animation: AnimationController::new(),
            base,
        }
    }
}
