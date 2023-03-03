// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

pub extern crate externc_libm as libm;

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use box2d_rs::b2_body::*;
use box2d_rs::b2_fixture::*;
use box2d_rs::b2_math::*;
use box2d_rs::b2_world::*;
use box2d_rs::b2rs_common::UserDataType;
use box2d_rs::shapes::b2_polygon_shape::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
struct UserDataTypes;

impl UserDataType for UserDataTypes {
    type Fixture = i32;
    type Body = i32;
    type Joint = i32;
}

pub fn hello_world() {
    // Define the gravity vector.
    let gravity = B2vec2::new(0.0, -10.0);

    // Construct a world object, which will hold and simulate the rigid bodies.
    let world = B2world::<UserDataTypes>::new(gravity);

    // Define the ground body.
    let mut ground_body_def = B2bodyDef::default();
    ground_body_def.position.set(0.0, -10.0);

    // Call the body factory which allocates memory for the ground body
    // from a pool and creates the ground box shape (also from a pool).
    // The body is also added to the world.
    let ground_body = B2world::create_body(world.clone(), &ground_body_def);

    // Define the ground box shape.
    let mut ground_box = B2polygonShape::default();

    // The extents are the half-widths of the box.
    ground_box.set_as_box(50.0, 10.0);

    // Add the ground fixture to the ground body.
    B2body::create_fixture_by_shape(ground_body, Rc::new(RefCell::new(ground_box)), 0.0);

    // Define the dynamic body. We set its position and call the body factory.
    let mut body_def = B2bodyDef::default();
    body_def.body_type = B2bodyType::B2DynamicBody;
    body_def.position.set(0.0, 4.0);
    let body = B2world::create_body(world.clone(), &body_def);

    // Define another box shape for our dynamic body.
    let mut dynamic_box = B2polygonShape::default();
    dynamic_box.set_as_box(1.0, 1.0);

    // Define the dynamic body fixture.
    let mut fixture_def = B2fixtureDef::default();
    fixture_def.shape = Some(Rc::new(RefCell::new(dynamic_box)));

    // Set the box density to be non-zero, so it will be dynamic.
    fixture_def.density = 1.0;

    // Override the default friction.
    fixture_def.friction = 0.3;

    // Add the shape to the body.
    B2body::create_fixture(body.clone(), &fixture_def);

    // Prepare for simulation. Typically we use a time step of 1/60 of a
    // second (60Hz) and 10 iterations. This provides a high quality simulation
    // in most game scenarios.
    let time_step: f32 = 1.0 / 60.0;
    let velocity_iterations: i32 = 6;
    let position_iterations: i32 = 2;

    let mut position: B2vec2 = body.borrow().get_position();
    let mut angle: f32 = body.borrow().get_angle();

    // This is our little game loop.
    for _i in 0..60 {
        // Instruct the world to perform a single step of simulation.
        // It is generally best to keep the time step and iterations fixed.
        world
            .borrow_mut()
            .step(time_step, velocity_iterations, position_iterations);

        // Now print the position and angle of the body.
        position = body.borrow().get_position();
        angle = body.borrow().get_angle();

        println!("{:4.2} {:4.2} {:4.2}", position.x, position.y, angle);
    }

    assert!(b2_abs(position.x) < 0.01);
    assert!(b2_abs(position.y - 1.01) < 0.01);
    assert!(b2_abs(angle) < 0.01);
}

pub fn main() {
    // Load the first number from the host
    let a: u64 = env::read();
    // Load the second number from the host
    let b: u64 = env::read();
    // Verify that neither of them are 1 (i.e. nontrivial factors)
    if a == 1 || b == 1 {
        panic!("Trivial factors")
    }

    let result: f32 = f32::sqrt(81.0);
    // let result: u32 = 10;
    // env::commit(&result);

    hello_world();

    // Compute the product while being careful with integer overflow
    let product = a.checked_mul(b).expect("Integer overflow");
    env::commit(&product);
}
