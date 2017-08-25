use collision::{Collider, Collidable};
use na::{Vector2, Vector4};
use games::Game;
use rendering::renderables::Renderable;
use input::mouse::MouseInput;
use input::keyboard::KeyboardInput;
use input::bool_switch::BoolSwitch;
use games;
use geometry::{ConPoly, Line, Circle, Point};
use std::slice::IterMut;
use collision::coll_obj_wrapper::{CollObjectWrapper, CollObjectWrapperTrait};

pub mod builder;

pub const RED: [f64; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: [f64; 4] = [0.0, 0.0, 1.0, 1.0];

pub struct CollisionTestGame {
    polys: Vec<CollObjectWrapper<ConPoly, CollisionTestObject>>,
    lines: Vec<CollObjectWrapper<Line, CollisionTestObject>>,
    circles: Vec<CollObjectWrapper<Circle, CollisionTestObject>>,
    points: Vec<CollObjectWrapper<Point, CollisionTestObject>>,
    collider: Collider,
    external_input: CollisionTestExternalGameInput,
    game_input: GameInput,
    mouse_mov: Vector2<f64>,
    mouse_speed: f64,
    player_index: usize,
}

#[derive(Clone, Default)]
pub struct CollisionTestExternalGameInput {
    mouse: MouseInput,
    kbd: KeyboardInput,
}

#[derive(Clone)]
pub enum CollisionTestObject {
    Circle,
    Line,
    Point,
    Poly
}

impl Game for CollisionTestGame {
    fn update_input(&mut self) {
        self.update_switches();
        self.update_object_controls();
        self.update_player_control();
    }

    fn update_logic(&mut self, t_step: f64) {
        self.set_mouse();
        let mov_horizontal = self.game_input.mov_horizontal;
        let mov_vertical = self.game_input.mov_vertical;
        let rot = self.game_input.rot;

        for obj in self.get_coll_objects_mut() {
            obj.set_prev();
            if obj.is_player_controlled() {
                obj.shift_by(t_step * Vector2::new(mov_horizontal as f64, mov_vertical as f64));
                obj.rotate(t_step * rot as f64);
            }
        }

        self.handle_collision();

        self.update_colors();
    }

    fn get_renderables(&self) -> Vec<Box<Renderable>> {
        let mut output: Vec<Box<Renderable>> = vec![];
        for obj in self.get_coll_objects() {
            output.push(obj.render(0.0));
        }
        output
    }

    fn get_input<'a>(&'a mut self) -> Option<&'a mut games::GameInput> {
        Some(&mut self.external_input)
    }
}

impl CollisionTestGame {
    fn handle_collision(&mut self) {
        let collidables: Vec<_> = self.get_collidables_mut().collect();
        Collider::process_all(collidables);
    }

    fn set_mouse(&mut self) {
        let ext_mouse_mov = self.external_input.mouse.movement;
        self.mouse_mov = Vector2::new(ext_mouse_mov.0 as f64, -ext_mouse_mov.1 as f64) * self.mouse_speed;
    }

    fn get_collidables_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut Collidable<Data = CollisionTestObject>> {
        self.polys.iter_mut().map(|it| -> &'a mut Collidable<Data = CollisionTestObject> {it})
        .chain(self.lines.iter_mut().map(|it| -> &'a mut Collidable<Data = CollisionTestObject> {it}))
        .chain(self.circles.iter_mut().map(|it| -> &'a mut Collidable<Data = CollisionTestObject> {it}))
        .chain(self.points.iter_mut().map(|it| -> &'a mut Collidable<Data = CollisionTestObject> {it}))    
    }

    fn get_coll_objects_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut CollObjectWrapperTrait<Data = CollisionTestObject>> {
        self.polys.iter_mut().map(|it| -> &'a mut CollObjectWrapperTrait<Data = CollisionTestObject> {it})
        .chain(self.lines.iter_mut().map(|it| -> &'a mut CollObjectWrapperTrait<Data = CollisionTestObject> {it}))
        .chain(self.circles.iter_mut().map(|it| -> &'a mut CollObjectWrapperTrait<Data = CollisionTestObject> {it}))
        .chain(self.points.iter_mut().map(|it| -> &'a mut CollObjectWrapperTrait<Data = CollisionTestObject> {it}))    
    }

    fn get_coll_objects<'a>(&'a self) -> impl Iterator<Item = &'a CollObjectWrapperTrait<Data = CollisionTestObject>> {
        self.polys.iter().map(|it| -> &'a CollObjectWrapperTrait<Data = CollisionTestObject> {it})
        .chain(self.lines.iter().map(|it| -> &'a CollObjectWrapperTrait<Data = CollisionTestObject> {it}))        
        .chain(self.circles.iter().map(|it| -> &'a CollObjectWrapperTrait<Data = CollisionTestObject> {it}))        
        .chain(self.points.iter().map(|it| -> &'a CollObjectWrapperTrait<Data = CollisionTestObject> {it}))        
    }

    fn get_total_objects(&self) -> usize {
        self.get_coll_objects().count()
    }

    fn update_switches(&mut self) {
        self.game_input.right_switch.update_state(self.external_input.kbd.right);
        self.game_input.left_switch.update_state(self.external_input.kbd.left);        
    }

    fn update_object_controls(&mut self) {
        self.game_input.mov_horizontal = self.external_input.kbd.d as isize - (self.external_input.kbd.a as isize);
        self.game_input.mov_vertical = self.external_input.kbd.w as isize - (self.external_input.kbd.s as isize);
        self.game_input.rot = self.external_input.kbd.e as isize - (self.external_input.kbd.q as isize);
    }

    fn update_player_control(&mut self) {
        self.update_player_index();
        
        let player_index = self.player_index;

        for obj in self.get_coll_objects_mut() {
            let player_controlled_flag = obj.get_object_index() == player_index;
            obj.set_player_control(player_controlled_flag);
            if player_controlled_flag {
                obj.reset_collision_flag();
            }
        }
    }

    fn update_player_index(&mut self) {
        if self.player_index == 0 && self.game_input.left_switch.pressed() {
            self.player_index = self.get_total_objects() - 1; 
            return;
        }
        if self.player_index == self.get_total_objects() - 1 && self.game_input.right_switch.pressed() {
            self.player_index = 0;
            return;
        }
        self.player_index += self.game_input.right_switch.pressed() as usize;
        self.player_index -= self.game_input.left_switch.pressed() as usize;
    }

    fn update_colors(&mut self) {
        let player_index = self.player_index;

        for obj in self.get_coll_objects_mut() {
            let player_controlled_flag = obj.get_object_index() == player_index;
            match (player_controlled_flag, obj.has_collided_in_past()) {
                (true, _) => obj.set_color(Vector4::new(1.0, 0.2, 0.2, 1.0)),
                (false, false) => obj.set_color(Vector4::new(1.0, 1.0, 1.0, 1.0)),
                (false, true) => obj.set_color(Vector4::new(0.2, 0.2, 1.0, 1.0))
            }
        }
    }
}

impl games::GameInput for CollisionTestExternalGameInput {
    fn get_mouse_inp<'a>(&'a mut self) -> Option<&'a mut MouseInput> { Some(&mut self.mouse) }
    fn get_kbd_inp<'a>(&'a mut self) -> Option<&'a mut KeyboardInput> { Some(&mut self.kbd) }
}

#[derive(Default, Debug)]
pub struct GameInput {
    right_switch: BoolSwitch,
    left_switch: BoolSwitch,
    mov_horizontal: isize,
    mov_vertical: isize,
    rot: isize
}