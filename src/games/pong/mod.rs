pub mod ball;
pub mod player;
pub mod paddle;
pub mod line;
pub mod board;
pub mod builder;

use self::ball::Ball;
use self::player::Player;
use self::board::Board;

use na::{Vec2, Vec4};
use collision;
use collision::{Collider, Collidable, CollResults};
use games;
use games::Game;
use rendering::renderables::Renderable;

pub const BACKGROUND_LAYER: f64 = 0.1;
pub const FOREGROUND_LAYER: f64 = 0.0;
pub const BOARD_COLOR: Vec4<f64> = Vec4 { x: 0.2, y: 0.2, z: 0.2, w: 1.0};
pub const BALL_COLOR: Vec4<f64> = Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0};
pub const RED: Vec4<f64> = Vec4 { x: 1.0, y: 0.0, z: 0.0, w: 1.0};
pub const BLUE: Vec4<f64> = Vec4 { x: 0.0, y: 0.0, z: 1.0, w: 1.0};

pub struct PongGame {
    balls: Vec<Ball>,
    players: Vec<Player>,
    board: Board,
    collider: Collider
}

#[derive(Clone)]
enum PongObject {
    Ball,
    Player
}

impl Game for PongGame {
    fn update_logic(&mut self, t_step: f64) {
        for ball in &mut self.balls {
            ball.update_pos(t_step);
        }
        self.handle_collision();
    }

    fn get_renderables(&self) -> Vec<Box<Renderable>> {
        let mut output: Vec<Box<Renderable>> = self.balls.iter().map(|x| {Box::new(x.render()) as Box<Renderable>})
            .chain(self.players.iter().map(|x| {Box::new(x.render()) as Box<Renderable>})).collect();
        output.push(Box::new(self.board.render()));
        output
    }
}

impl PongGame {
    fn handle_collision(&mut self) {
        let collidables: Vec<_> = self.balls.iter_mut().map(|x| {x as &mut Collidable<Data=PongObject>})
            .chain(self.players.iter_mut().map(|x| {x as &mut Collidable<Data=PongObject>})).collect();
        self.collider.process_all(collidables);
    }
}
