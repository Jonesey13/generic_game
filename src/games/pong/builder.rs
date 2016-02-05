use na::Vec2;
use num::Zero;
use super::PongGame;
use super::ball::Ball;
use super::player::Player;
use super::paddle::Paddle;
use super::board::Board;
use super::line::Line;
use collision::Collider;

pub struct PongBuilder {
    board_dim: Vec2<f64>,
    ball_size: f64,
    ball_velocity: Vec2<f64>,
    paddle_size: Vec2<f64>,
    paddle_speed: f64,
}

impl Default for PongBuilder {
    fn default() -> Self {
        PongBuilder {
            board_dim: Vec2::new(1.5, 1.0),
            ball_size: 0.02,
            ball_velocity: Vec2::new(0.1, 0.0),
            paddle_size: Vec2::new(0.1, 0.02),
            paddle_speed: 0.02
        }
    }
}

impl PongBuilder {
    pub fn init() -> PongBuilder {
        Self::default()
    }

    pub fn with_board_dim<'a>(&'a mut self, len: f64, wid: f64) -> &'a mut PongBuilder {
        self.board_dim = Vec2::new(len, wid); self
    }

    pub fn with_ball_size<'a>(&'a mut self, radius: f64) -> &'a mut PongBuilder {
        self.ball_size = radius; self
    }

    pub fn with_ball_velocity<'a>(&'a mut self, velocity: Vec2<f64>) -> &'a mut PongBuilder {
        self.ball_velocity = velocity; self
    }

    pub fn with_paddle_size<'a>(&'a mut self, sizes: Vec2<f64>) -> &'a mut PongBuilder {
        self.paddle_size = sizes; self
    }

    pub fn with_paddle_speed<'a>(&'a mut self, speed: f64) -> &'a mut PongBuilder {
        self.paddle_speed = speed; self
    }

    pub fn build_game(&mut self) -> PongGame {
        let mut ball = Ball::new(Vec2::zero(), self.ball_size, super::BALL_COLOR);
        ball.set_velocity(self.ball_velocity);
        let half_bdim = self.board_dim / 2.0;

        let line_1 = Line::new(
            Vec2::new( -half_bdim.x, - half_bdim.y ),
            Vec2::new( -half_bdim.x, half_bdim.y));
        let player_1 = Player::new(1, Paddle::new(self.paddle_size, super::RED), line_1);

        let line_2 = Line::new(
            Vec2::new( half_bdim.x, - half_bdim.y ),
            Vec2::new( half_bdim.x, half_bdim.y));
        let player_2 = Player::new(1, Paddle::new(self.paddle_size, super::BLUE), line_2);

        PongGame {
            balls: vec![ball],
            players: vec![player_1, player_2],
            board: Board::new(self.board_dim.x, self.board_dim.y, super::BOARD_COLOR),
            collider: Collider,
        }

    }
}
