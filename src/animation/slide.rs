use crate::{
    core::application::request_animation_frame,
    core::application::Application,
    graphics::shapes::{IrregularPolygon, Shape},
};
use keyframe::{
    ease,
    functions::{EaseIn, EaseInOut},
    keyframes,
    num_traits::Float,
    AnimationSequence, EasingFunction,
};
use std::{
    cell::{RefCell, RefMut},
    ops::AddAssign,
    rc::Rc,
};
use wasm_bindgen::prelude::Closure;
#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    fn move_to(&self, shape: RefMut<impl Shape>, t: f32) {
        match self {
            Axis::X => shape.move_to(t, 0.0),
            Axis::Y => shape.move_to(0.0, t),
        };
    }
}
pub fn slide_axis(
    shape: Rc<RefCell<impl Shape>>,
    animation_duration: u32,
    mut sequence: RefMut<AnimationSequence<f32>>,

    axis: Axis,
) {
    sequence.advance_by(1.0 / (animation_duration as f64 * 60.0));

    axis.move_to(shape.borrow_mut(), sequence.now());
}

pub fn slide(
    shape: Rc<RefCell<impl Shape + 'static>>,
    from: i32,
    to: i32,
    animation_duration: u32,
    function: impl EasingFunction + 'static + Send + Sync,
    axis: Axis,
    app: Rc<Application>,
) {
    let sequence = keyframes![(from as f32, 0.0, function), (to as f32, 1.0, EaseInOut)];

    let temp = Rc::new(RefCell::new(sequence));
    let temp2 = temp.clone();
    let closure = move || {
        app.render();
        slide_axis(shape.clone(), animation_duration, temp2.borrow_mut(), axis);
    };

    render_loop_anim(closure, temp.clone());
}

pub fn render_loop_anim<F>(mut closure: F, sequence: Rc<RefCell<AnimationSequence<f32>>>)
where
    F: 'static + FnMut(),
{
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        closure();
        if !sequence.borrow().finished() {
            request_animation_frame(f.borrow().as_ref().unwrap());
        }
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}
