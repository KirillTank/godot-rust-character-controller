# Godot Rust Character Controller

A reusable 2D character controller for Godot 4.7 built with Rust and rust-godot.

This project demonstrates a clean implementation of a 2D platformer controller using a finite state machine (FSM), animation management, and input handling.

---

## Features

### Movement

* Horizontal movement
* Gravity system
* Jump mechanics
* Air movement

### Finite State Machine

The controller uses a state machine to manage character behavior.

Available states:

* Idle
* Run
* Fall
* Attack

### Animation System

* Sprite animation playback
* AnimationPlayer support
* Animation state synchronization
* Horizontal sprite flipping

### Input Handling

Supported actions:

| Action     | Input Action |
| ---------- | ------------ |
| Move Left  | left         |
| Move Right | right        |
| Jump       | jump         |

---

Main components:

### State

Responsible for character state transitions.

```rust
enum State {
    Idle,
    Run,
    Fall,
}
```

### AnimationController

Handles:

* Sprite playback
* AnimationPlayer playback
* Animation state tracking
* Signal integration

### MovementController

Handles:

* Velocity
* Gravity
* Jump force
* Horizontal movement
* Input values

---

## Requirements

* Godot 4.7
* Rust
* rust-godot 0.5.x

---

## Scene Setup

Required node structure:

```text
Player (CharacterBody2D)
├── anim
│   ├── Player (AnimationPlayer)
│   ├── Sprite (AnimatedSprite2D)
│   └── Area2D
```

Input Map:

```text
left
right
jump
```
<img width="1280" height="720" alt="output" src="https://github.com/user-attachments/assets/73b32c1e-fde5-422f-8d49-dfa1898e6a43" />
---

## Demo

Add a GIF or video preview here.

Recommended showcase:

* Idle
* Run
* Jump
* Fall

---


---

## License

MIT
