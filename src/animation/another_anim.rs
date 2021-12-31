use std::{cell::RefCell, rc::Rc};

use keyframe::{functions::EaseInOut, keyframes, AnimationSequence, EasingFunction};

use crate::{
    graphics::{scene_graph::GraphNode, Shape},
    utils::console_log,
};

use super::slide::Axis;

pub struct SlideAnim {
    node_ref: Rc<RefCell<GraphNode>>,
    sequence: AnimationSequence<f32>,
    duration: f64,
    axis: Axis,
}

impl SlideAnim {
    pub fn new(
        node_ref: Rc<RefCell<GraphNode>>,
        sequence: AnimationSequence<f32>,
        duration: f32,
        axis: Axis,
    ) -> Self {
        SlideAnim {
            node_ref,
            sequence,
            duration: duration as f64,
            axis,
        }
    }

    pub fn tick(&mut self, elapsed_since_last_call: f64) {
        if self.sequence.finished() {
            return;
        }

        let curr_val = self.sequence.now();

        match self.axis {
            Axis::X => self.node_ref.borrow_mut().geom.move_to_x(curr_val),
            Axis::Y => self.node_ref.borrow_mut().geom.move_to_y(curr_val),
        };

        self.sequence.advance_by(elapsed_since_last_call);
    }

    pub fn done(&self) -> bool {
        self.sequence.finished()
    }
}

pub trait Animate: Shape {
    fn slide(
        &self,
        from: f32,
        to: f32,
        duration: f32,
        function: impl EasingFunction + 'static + Send + Sync,
        axis: Axis,
    ) -> SlideAnim {
        let seq = keyframes![
            (from as f32, 0.0, function),
            (to as f32, duration as f64, EaseInOut)
        ];

        SlideAnim::new(self.get_node(), seq, duration, axis)
    }
}
