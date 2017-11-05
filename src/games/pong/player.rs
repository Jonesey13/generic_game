use na::{Vector1, Vector2, Vector3, Vector4, Rotation2};
use super::paddle::Paddle;
use geometry::line::Line;
use geometry::con_poly;
use rendering;
use collision;
use collision::{Collidable, CollObj, CollisionObjectResults};
use std::f64::consts::PI;
use super::FOREGROUND_LAYER;

pub struct Player {
    num: usize,
    slide_pos: f64,
    paddle: Paddle,
    line: Line,
    coll_results: CollisionObjectResults<super::PongObject>,
    prev: Option<Box<Player>>
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            num: self.num,
            slide_pos: self.slide_pos,
            paddle: self.paddle.clone(),
            line: self.line.clone(),
            coll_results: self.coll_results.clone(),
            prev: None
        }
    }
}

impl Player {
    pub fn new(num: usize, paddle: Paddle, line: Line) -> Player {
        Player {
            num: num,
            slide_pos: 0.5,
            paddle: paddle,
            line: line,
            coll_results: CollisionObjectResults::no_collision(),
            prev: None
        }
    }

    pub fn set_slide_position(&mut self, pos: f64) {
        self.slide_pos = pos;
    }

    pub fn get_position(&self) -> Vector2<f64> {
        self.line.get_point(self.slide_pos)
    }

    pub fn get_current_poly(&self) -> con_poly::ConPoly {
        con_poly::ConPoly::new_from_rect(self.paddle.width, self.paddle.length, self.get_position(), self.get_rotation())
    }

    pub fn get_previous_poly(&self) -> con_poly::ConPoly {
        if let Some(ref prev_player) = self.prev {
            return con_poly::ConPoly::new_from_rect(prev_player.paddle.width, prev_player.paddle.length, prev_player.get_position(), prev_player.get_rotation());
        }
        self.get_current_poly()
    }

    /// Convention: roatation angle 0 = Vertical Paddle on Left Side
    pub fn get_rotation(&self) -> Rotation2<f64> {
        let dir: Vector2<f64> = self.line.end - self.line.beg;
        let clockwise_angle = -dir.y.atan2(dir.x);
        let rot_angle = clockwise_angle + PI / 2.0;
        Rotation2::new(rot_angle)
    }

    pub fn render(&self) -> rendering::rectangle::Rectangle {
        let height = self.paddle.length; // Intentional: default is tall
        let length = self.paddle.width;
        let point_2d = self.line.get_point(self.slide_pos);
        rendering::rectangle::Rectangle {
            length: length,
            height: height,
            rot: self.get_rotation(),
            pos: Vector3::new(point_2d.x, point_2d.y, FOREGROUND_LAYER),
            color: self.paddle.color,
        }
    }
}

impl Collidable for Player {
    type Data = super::PongObject;

    fn get_collision_object(&self) -> CollObj {
        CollObj::ConPoly(self.get_current_poly(), self.get_previous_poly())
    }

    fn get_collision_object_results(&self) -> CollisionObjectResults<Self::Data> {
        self.coll_results.clone()
    }

    fn set_collision_object_results(&mut self, new_results: CollisionObjectResults<Self::Data>) {
        self.coll_results = new_results;
    }

    fn get_collision_data(&self) -> Self::Data { super::PongObject::Player }
}
