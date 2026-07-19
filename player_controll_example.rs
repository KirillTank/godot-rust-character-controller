use godot::classes::{
    AnimatedSprite2D, AnimationPlayer, Area2D, CharacterBody2D, ICharacterBody2D, Input,
};
use godot::global::Error;
use godot::prelude::*;

/// Character states used by the finite state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Idle,
    Run,
    Fall,
}

/// Handles sprite and animation playback.
struct AnimationController {
    player: Option<Gd<AnimationPlayer>>,
    sprite: Option<Gd<AnimatedSprite2D>>,
    attack_area: Option<Gd<Area2D>>,

    current_sprite: String,
    animation_finished: bool,
}

impl AnimationController {
    fn new() -> Self {
        Self {
            player: None,
            sprite: None,
            attack_area: None,
            current_sprite: String::new(),
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
}

/// Runtime movement data.
struct MovementController {
    velocity_x: f32,
    velocity_y: f32,

    direction: f32,

    speed: f32,
    gravity: f32,
    jump_force: f32,
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
            state: State::Idle,
            animation: AnimationController::new(),
            movement: MovementController::new(),
            base,
        }
    }

    fn ready(&mut self) {
        // Инициализация узлов из сцены
        self.animation.player = self.init_node::<AnimationPlayer>("anim/Player");
        self.animation.sprite = self.init_node::<AnimatedSprite2D>("anim/Sprite");
        self.animation.attack_area = self.init_node::<Area2D>("anim/Area2D");

        // Подключаем сигнал окончания анимации
        let owner = self.to_gd();
        if let Some(player) = &self.animation.player {
            Player::connect_signal_un(
                player,
                "animation_finished",
                &owner,
                "_on_animation_finished",
            );
        }
    }

    fn physics_process(&mut self, delta: f32) {
        self.process_input(delta);
        self.update_state(delta);

        let vel = self.movement.velocity();
        self.base_mut().set_velocity(vel);
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Player {
    fn init_node<T>(&mut self, path: &str) -> Option<Gd<T>>
    where
        T: GodotClass + Inherits<Node>,
    {
        let node = self.base().try_get_node_as(path);
        if node.is_none() {
            godot_print!("Node {} not found!", path);
        }
        node
    }

    fn process_input(&mut self, _delta: f32) {
        let input = Input::singleton();
        self.movement.direction = input.get_axis("left", "right");

        if input.is_action_just_pressed("jump") && self.base().is_on_floor() {
            self.movement.velocity_y = self.movement.jump_force;
        }
    }

    fn update_state(&mut self, delta: f32) {
        //let prev_state = self.state;

        if !self.base().is_on_floor() {
            self.state = State::Fall;
        } else if self.movement.direction.abs() > 0.7 {
            self.state = State::Run;
        } else {
            self.state = State::Idle;
        }

        // Обновляем анимации по состоянию
        match self.state {
            State::Idle => {
                self.animation.play_sprite("idle");
                self.movement.velocity_x = 0.0;
            }
            State::Run => {
                self.movement.velocity_x = self.movement.direction * self.movement.speed;
                self.animation.play_sprite("run");
            }
            State::Fall => {
                self.movement.velocity_x = self.movement.direction * self.movement.speed;
                self.movement.velocity_y += self.movement.gravity * delta;
                self.animation.play_sprite("fall");
            }
        }

        // Если поменялось направление, меняем флип
        self.update_facing();
    }

    fn update_facing(&mut self) {
        if let Some(sprite) = &mut self.animation.sprite {
            let movment = self.movement.direction;

            if movment.abs() > 0.1 {
                let should_flip = movment < 0.0;

                if sprite.is_flipped_h() != should_flip {
                    sprite.set_flip_h(should_flip);
                }
            }
        }
    }

    fn connect_signal_un<E, T>(
        emitter: &Gd<T>,
        signal_name: &str,
        target: &Gd<E>,
        method_name: &str,
    ) where
        E: GodotClass + Inherits<Object>,
        T: GodotClass + Inherits<Object>,
    {
        let sig = Signal::from_object_signal(emitter, signal_name);
        let call = Callable::from_object_method(target, method_name);
        match sig.connect(&call) {
            Error::OK => {}
            err => godot_error!("connect failed {:?} {}", err, signal_name),
        }
    }

    #[func]
    fn _on_animation_finished(&mut self, _name: StringName) {
        self.animation.animation_finished = true;
    }
}
