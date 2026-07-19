# Godot Rust Character Controller

A reusable 2D character controller for Godot 4 written in Rust using rust-godot.

This project demonstrates a clean separation between character movement, animation handling, and state management. The controller is built around a finite state machine (FSM) architecture, making it easy to extend and integrate into platformer, action, or RPG projects.

## Features

* Finite State Machine (FSM)
* Idle State
* Run State
* Fall State
* Attack State
* Animation Controller
* Sprite Animation Management
* CharacterBody2D Integration
* Modular Architecture
* Written entirely in Rust

## Architecture

### AnimationController

Responsible for:

* Sprite animation playback
* AnimationPlayer control
* Animation state tracking
* Character orientation checks

### MovementController

Responsible for:

* Horizontal movement
* Gravity simulation
* Jump configuration
* Velocity calculations
* Input-driven movement data

### Player

Main gameplay controller that combines movement, animation, and state management into a single reusable component.

## Technologies

* Rust
* Godot 4
* rust-godot

## Project Goals

This project was created to explore game development with Rust and demonstrate practical usage of:

* State machines
* Godot node interaction
* Animation systems
* Modular game architecture
* Rust game programming patterns

## Future Improvements

* Jump State
* Death State
* Combo Attacks
* Air Control
* Save System Integration
* Multiplayer Support
