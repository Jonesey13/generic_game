use na::{Vector2, Rotation2};
use num::Zero;
use super::{PhysicsTestGame, PhysicsTestGameInput};
use super::coll_circle::CollCircle;
use super::coll_rect::CollRect;
use super::BLUE;
use gg::collision::Collider;

pub struct PhysicsTestBuilder {
    curr_circ : Option<CollCircle>,
    circles: Vec<CollCircle>,
    curr_rect: Option<CollRect>,
    rects: Vec<CollRect>
}

impl Default for PhysicsTestBuilder {
    fn default() -> Self {
        PhysicsTestBuilder {
            curr_circ: None,
            circles: Vec::new(),
            curr_rect: None,
            rects: Vec::new()
        }
    }
}

impl PhysicsTestBuilder {
    pub fn init() -> PhysicsTestBuilder {
        Self::default()
    }

    fn clear_currents(&mut self) {
        self.curr_circ = None;
        self.curr_rect = None;
    }

    fn push_existing(&mut self) {
        if let Some(ref old_circ) = self.curr_circ {
            self.circles.push(old_circ.clone());
        }
        if let Some(ref old_rect) = self.curr_rect {
            self.rects.push(old_rect.clone());
        }
        self.clear_currents();
    }

    pub fn add_circle<'a> (&'a mut self, pos: Vector2<f64>, rad: f64) -> &'a mut Self {
        self.push_existing();
        self.curr_circ = Some(CollCircle::new(pos, rad, BLUE.into()));
        self
    }

    pub fn add_rect<'a> (&'a mut self, pos: Vector2<f64>, length: f64, height: f64, rot: Rotation2<f64>) -> &'a mut Self {
        self.push_existing();
        self.curr_rect = Some(CollRect::new(pos, length, height, rot, BLUE.into()));
        self
    }

    pub fn with_velocity<'a> (&'a mut self, vel: Vector2<f64>) -> &'a mut Self {
        if let Some(ref mut circ) = self.curr_circ {
            circ.set_velocity(vel);
        }
        if let Some(ref mut rect) = self.curr_rect {
            rect.set_velocity(vel);
        }
        self
    }

    pub fn player_controlled<'a> (&'a mut self) -> &'a mut Self {
        if let Some(ref mut circ) = self.curr_circ {
            circ.toggle_player_control();
        }
        if let Some(ref mut rect) = self.curr_rect {
            rect.toggle_player_control();
        }
        self
    }

    pub fn build_game(&mut self) -> PhysicsTestGame {
        self.push_existing();

        PhysicsTestGame {
            circles: self.circles.clone(),
            rects: self.rects.clone(),
            collider: Collider,
            external_input: Default::default(),
            mouse_mov: Vector2::zero(),
            mouse_speed: 0.01
        }
    }
}
