use na::Vec2;
use num::Zero;
use super::{PhysicsTestGame, PhysicsTestGameInput};
use super::coll_circle::CollCircle;
use collision::Collider;

pub struct PhysicsTestBuilder {
    circle1_rad: f64,
    circle1_pos: Vec2<f64>,
    circle1_vel: Vec2<f64>,
    circle2_rad: f64,
    circle2_pos: Vec2<f64>,
    circle2_vel: Vec2<f64>,
}

impl Default for PhysicsTestBuilder {
    fn default() -> Self {
        PhysicsTestBuilder {
            circle1_rad: 0.1,
            circle1_pos: Vec2::new(-0.5, 0.0),
            circle1_vel: Vec2::new(0.5, 0.0),
            circle2_rad: 0.1,
            circle2_pos: Vec2::new(0.5, 0.0),
            circle2_vel: Vec2::new(-0.5, 0.0),
        }
    }
}

impl PhysicsTestBuilder {
    pub fn init() -> PhysicsTestBuilder {
        Self::default()
    }

    pub fn build_game(&mut self) -> PhysicsTestGame {
        let mut circle1 = CollCircle::new(self.circle1_pos, self.circle1_rad, super::BLUE);
        circle1.toggle_player_control();
        let mut circle2 = CollCircle::new(self.circle2_pos, self.circle2_rad, super::BLUE);
        circle2.set_velocity(self.circle2_vel);

        PhysicsTestGame {
            circles: vec![circle1, circle2],
            collider: Collider,
            external_input: Default::default()
        }
    }
}
