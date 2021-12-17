use std::ops::AddAssign;

use keyframe::{ease_with_scaled_time, AnimationSequence, EasingFunction};

use crate::{
    graphics::shapes::{IrregularPolygon, Shape},
    puts,
};

pub enum Axis {
    Xneg,
    X,
    Yneg,
    Y,
}

impl Axis {
    fn translate(&self, shape: &mut IrregularPolygon, t: f32) {
        match self {
            Axis::X => shape.translate2(t, 0.0),
            Axis::Y => shape.translate2(0.0, t),
            Axis::Xneg => shape.translate2(-t, 0.0),
            Axis::Yneg => shape.translate2(0.0, -t),
        };
    }

    fn get_position(&self, shape: &IrregularPolygon) -> f32 {
        match self {
            &Axis::X | &Axis::Xneg => return shape.x as f32,
            &Axis::Y | &Axis::Yneg => return shape.y as f32,
        }
    }
}
pub fn slide_anim(
    shape: &mut IrregularPolygon,
    final_position: i32,
    time_step: u32,
    sequence: &mut AnimationSequence<f32>,
    axis: Axis,
) {
    sequence.advance_by(1.0 / time_step as f64);

    let x = axis.get_position(shape);

    //puts(&sequence.now().to_string());
    match x {
        d if (d - final_position as f32).abs() < sequence.now() => return,
        _ => axis.translate(shape, sequence.now()),
    }
}
