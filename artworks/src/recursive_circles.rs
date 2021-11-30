use nannou::{prelude::*, ui::utils::modulo};

use crate::artwork::Artwork;

const WIDTH: u32 = 1850;
const HEIGHT: u32 = 1010;

const STARTING_RADIUS: f32 = 250.0;

const RECURSION_LEVELS: u32 = 10;
const RECURSIVE_REDUCTION: f32 = 0.69;

const RAND_PLACEMENT_SCALE : f32 = 20.0;
const RAND_RADIUS_SCALE: f32 = 24.0;
const RAND_DRAW_CHANCE: f32 = 0.33;
const RAND_BRANCH_CHANCE: f32 = 0.6;

pub struct Model {}

fn rec_draw_circle(draw: &Draw, x: f32, y: f32, radius: f32, level: u32) {
    if random_f32() <= RAND_DRAW_CHANCE {
        draw.ellipse()
            .color(hsla(
                0.15 + (modulo(x.abs() as u32, 64) as f32 / 400.0),
                0.06 + (modulo(y.abs() as u32, 128) as f32 / 255.0),
                0.06 + (modulo(radius.abs() as u32, 54) as f32 / 800.0) + (level as f32 / (RECURSION_LEVELS + 3) as f32),
                1.2 - level as f32 / RECURSION_LEVELS as f32)
            )
            .x(x)
            .y(y)   
            .w_h(radius * 1.9, radius * 2.1);
    }

    if level > RECURSION_LEVELS {
        return
    }
    else {
        if random_f32() <= RAND_BRANCH_CHANCE {
            rec_draw_circle(
                &draw.rotate(0.16),
                x + radius + (0.5 - random_f32()) * RAND_PLACEMENT_SCALE,
                y + (0.5 - random_f32()) * RAND_PLACEMENT_SCALE,
                radius * RECURSIVE_REDUCTION + (0.5 - random_f32()) * RAND_RADIUS_SCALE, 
                level + 1);
        }
        if random_f32() <= RAND_BRANCH_CHANCE {
            rec_draw_circle(
                &draw.rotate(-0.064),
                x - radius + (0.5 - random_f32()) * RAND_PLACEMENT_SCALE,
                y + (0.5 - random_f32()) * RAND_PLACEMENT_SCALE,
                radius * RECURSIVE_REDUCTION + (0.5 - random_f32()) * RAND_RADIUS_SCALE, 
                level + 1);
        }
        if random_f32() <= RAND_BRANCH_CHANCE {
            rec_draw_circle(
                &draw.rotate(0.12),
                x + (0.5 - random_f32()) * RAND_PLACEMENT_SCALE,
                y - radius + (0.5 - random_f32()) * RAND_PLACEMENT_SCALE,
                radius * RECURSIVE_REDUCTION + (0.5 - random_f32()) * RAND_RADIUS_SCALE, 
                level + 1);
        }
        if random_f32() <= RAND_BRANCH_CHANCE {
            rec_draw_circle(
                &draw.rotate(-0.093),
                x + (0.5 - random_f32()) * RAND_PLACEMENT_SCALE,
                y + radius + (0.5 - random_f32()) * RAND_PLACEMENT_SCALE,
                radius * RECURSIVE_REDUCTION + (0.5 - random_f32()) * RAND_RADIUS_SCALE, 
                level + 1);
        }
    }
}

impl Artwork<Model> for Model {
    fn setup(&self) {
        nannou::sketch(Self::view) //TODO: figure out this. a case for having the "Model" here be another required implementation? looks like
            .size(WIDTH, HEIGHT)
            .run();
    }

    fn model(_app: &App) -> Model{
        Model {}
    }

    fn update(_app: &App, _model: &mut Model, _update: Update) {}
    
    fn view(app: &App, _model: &Model, frame: Frame) {
        app.set_loop_mode(LoopMode::loop_once());

        let draw = app.draw();
                            
        draw.background().color(rgb(0.05, 0.05, 0.05));
        
        rec_draw_circle(&draw, 0.0, 0.0, STARTING_RADIUS, 0);

        draw.to_frame(app, &frame).unwrap();
    }
}