/*
Central file for the polar_game module
*/

mod player;
pub mod object;
mod enemy;
mod flare;
mod sun;
mod frame;
pub mod builder;
pub use self::builder::PolarGameBuilder;


use self::player::Player;
use self::object::{Part,Object,Point,collision};
use self::flare::Flare;
use self::sun::Sun;
use self::enemy::Enemy;
use self::frame::PolarFrame;
use time;
use rand;
use rand::distributions::exponential::Exp;
use rand::distributions::IndependentSample;
use rand::distributions::range::Range;
use games::{GameInput, Game};
use input::keyboard::KeyboardInput;
use super::view_details::{PolarViewDetails, ViewDetails};
use rendering::renderables::Renderable;
use rendering::polar_pixel::PolarPixel;
use rendering::text::PlainText;

pub struct PolarGame{
    player: Player,
    flares: Vec<Flare>,
    sun: Sun,
    pub input_keys: InputKeys,
    frame: PolarFrame,
    pub setup: GameSetup,
    time: Times,
    pub state: GameState,
    external_input: ExternalInput,
    view_details: PolarViewDetails
}

impl PolarGame {
    pub fn new(setup: GameSetup) -> PolarGame{
        PolarGame{
            player: Player::new(setup.player_start, setup.player_width),
            flares: Vec::new(),
            sun: Sun::new(1.0),
            input_keys: InputKeys{
                jump_angle: 0.0,
                jump_radial: 0.0,
            },
            time: Times::new(0.0),
            frame: PolarFrame::new(0.5, 0.05, Point{x: 0.01, y: 0.02}, setup.radial_max),
            setup: setup,
            state: GameState::new(),
            external_input: Default::default(),
            view_details:  Default::default()
        }
    }

    fn update_view_details(&mut self) {}
}

impl Game for PolarGame {
    fn init(&mut self) {
        self.time = Times::new(time::precise_time_s());
    }

    fn update_input(&mut self) {
        self.input_keys.jump_radial = (self.external_input.kbd.up as isize - (self.external_input.kbd.down as isize)) as f64;
        self.input_keys.jump_angle = (self.external_input.kbd.right as isize - (self.external_input.kbd.left as isize)) as f64;
    }

    fn update_logic(&mut self, t_step: f64){
        let shift = Point{x: self.input_keys.jump_radial,
                          y: self.input_keys.jump_angle / 2.0};
        self.time.elapsed += t_step;

        self.player.update_position(shift, t_step, self.setup);
        for mut f in self.flares.iter_mut(){
            f.update_position(t_step, &self.player);
            if collision(&*f, &self.player){
                self.player.collide();;
            }
        }
        if collision(&self.sun, &self.player){
            self.player.collide();;
        }

        let current_flares = self.flares.clone();
        let (_, flares_trimmed) : (Vec<Flare>, Vec<Flare>)
            = current_flares.into_iter().partition(|f| f.terminate_flag(Point{x: -1.0, y: self.setup.radial_max + 2.0}));
        self.flares = flares_trimmed;


        if self.time.elapsed - self.time.previous_flare > self.time.til_flare{
            let mut rng = rand::thread_rng();
            let unif = Range::new(0.0, 1.0);
            let sa = unif.ind_sample(&mut rng);
            let r = unif.ind_sample(&mut rng) / 20.0 + 0.02;
            let a = unif.ind_sample(&mut rng) / 50.0 + 0.005;
            let v = unif.ind_sample(&mut rng) / 2.0 + 0.1;
            let new_flare = Flare::new(Point{x: r, y: a}, sa, v);
            self.flares.push(new_flare);
            self.time.previous_flare = self.time.elapsed;
            let emit_average = 10.0 + self.time.elapsed - self.time.start;
            let exp = Exp::new(emit_average);
            self.time.til_flare = exp.ind_sample(&mut rng);
        }

        let mut new_survival_time = self.state.survival_time;;
        if !self.player.destroyed{
            new_survival_time = self.time.elapsed - self.time.survival_start;
        }
        self.state = GameState{player_death: self.player.destroyed,
                               survival_time: new_survival_time,
        };

        self.update_view_details;
    }

    fn get_view(&self) -> ViewDetails {
        ViewDetails::Polar(self.view_details.clone())
    }

    fn get_renderables(&self) -> Vec<Box<Renderable>> {
        let mut rend_vec: Vec<Box<Renderable>> = Vec::new();
        for f in self.frame.get_render_parts().iter(){
            rend_vec.push(Box::new(PolarPixel::from(f.clone())));
        }
        for f in self.player.get_render_parts().iter(){
            rend_vec.push(Box::new(PolarPixel::from(f.clone())));
        }
        let sun_part = self.sun.get_render_parts()[0];
        rend_vec.push(Box::new(PolarPixel::from(sun_part)));
        for f in self.flares.iter(){
            let flare_part = f.get_render_parts()[0];
            rend_vec.push(Box::new(PolarPixel::from(flare_part)));
        }
        rend_vec
    }

    fn get_input<'a>(&'a mut self) -> Option<&'a mut GameInput> {
         Some(&mut self.external_input)
    }
}

pub struct InputKeys{
    pub jump_angle: f64,
    pub jump_radial: f64
}

#[derive(Copy, Clone)]
pub struct GameSetup{
    pub radial_max: f64,
    pub player_start: Point,
    pub player_width: Point,
}

impl Default for GameSetup {
    fn default() -> Self {
        GameSetup {
            radial_max: 8.0,
            player_start: Point{x: 4.0, y: 0.75},
            player_width: Point{x: 0.02, y: 0.01},
        }
    }
}

#[derive(Copy, Clone)]
pub struct GameState{
    pub player_death: bool,
    pub survival_time: f64,
}

impl GameState{
    pub fn new() -> GameState{
        GameState{ player_death: false,
                   survival_time: 0.0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Times{
    til_flare: f64,
    previous_flare: f64,
    start: f64,
    survival_start: f64,
    elapsed: f64,
}

impl Times{
    pub fn new(start_time: f64) -> Times{
        let mut rng = rand::thread_rng();
        let exp = Exp::new(1.0);
        Times{ til_flare: exp.ind_sample(&mut rng),
               previous_flare: start_time,
               start: start_time,
               survival_start: start_time,
               elapsed: start_time,
        }
    }
}

#[derive(Clone, Default)]
struct ExternalInput {
    kbd: KeyboardInput,
}

impl GameInput for ExternalInput {
    fn get_kbd_inp<'a>(&'a mut self) -> Option<&'a mut KeyboardInput> { Some(&mut self.kbd) }
}
