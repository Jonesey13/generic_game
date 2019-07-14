use crate::collision::{Collider, Collidable};
use crate::games::Game;
use crate::rendering::*;
use crate::input::mouse::MouseInput;
use crate::input::keyboard::KeyboardInput;
use crate::input::bool_switch::BoolSwitch;
use crate::games;
use crate::geometry::{ConPoly, Line, Circle, Point};
use std::slice::IterMut;
use crate::collision::collidable_wrapper::{CollidableWrapper, CollidableWrapperTrait};
use crate::collision::CollisionDataType;

pub mod builder;

pub const RED: [f64; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLUE: [f64; 4] = [0.0, 0.0, 1.0, 1.0];

pub struct CollisionTestGame {
    polys: Vec<CollidableWrapper<ConPoly, CollisionTestObject>>,
    lines: Vec<CollidableWrapper<Line, CollisionTestObject>>,
    circles: Vec<CollidableWrapper<Circle, CollisionTestObject>>,
    points: Vec<CollidableWrapper<Point, CollisionTestObject>>,
    collider: Collider,
    external_input: CollisionTestExternalGameInput,
    game_input: GameInput,
    mouse_mov: Point,
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

impl CollisionDataType for CollisionTestObject {}

impl Game for CollisionTestGame {
    type Primitive = StandardPrimitive;

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

        for obj in self.get_collision_wrappers_mut() {
            obj.set_prev();
            if obj.is_player_controlled() {
                obj.shift_by(t_step * 0.4 * Point::new(mov_horizontal as f64, mov_vertical as f64));
                obj.rotate_at_center(t_step * rot as f64);
            }
        }

        self.handle_collision();

        self.update_colors();
    }

    fn get_renderables(&mut self, _: WindowSpec) -> Vec<Box<StandardRenderable>> {
        let mut output: Vec<Box<StandardRenderable>> = vec![];
        for obj in self.get_collision_wrappers() {
            output.append(&mut obj.render(0.0));
            output.append(&mut obj.render_coll_results(-0.1));
        }
        output
    }

    fn get_input<'a>(&'a mut self) -> Option<&'a mut dyn games::GameInput> {
        Some(&mut self.external_input)
    }
}

impl CollisionTestGame {
    fn handle_collision(&mut self) {
        let collidables: Vec<_> = self.get_collidables_mut().collect();
        Collider::process_all(collidables);
    }

    fn set_mouse(&mut self) {
        if let Some(mouse) = self.external_input.mouse.devices.iter().nth(0) {
            let ext_mouse_mov = mouse.movement;
            self.mouse_mov = self.mouse_speed * Point::new(ext_mouse_mov.0 as f64, -ext_mouse_mov.1 as f64);
        }
    }

    fn get_collidables_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut dyn Collidable<Data = CollisionTestObject>> {
        self.polys.iter_mut().map(|it| -> &'a mut dyn Collidable<Data = CollisionTestObject> {it})
        .chain(self.lines.iter_mut().map(|it| -> &'a mut dyn Collidable<Data = CollisionTestObject> {it}))
        .chain(self.circles.iter_mut().map(|it| -> &'a mut dyn Collidable<Data = CollisionTestObject> {it}))
        .chain(self.points.iter_mut().map(|it| -> &'a mut dyn Collidable<Data = CollisionTestObject> {it}))    
    }

    fn get_collision_wrappers_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut dyn CollidableWrapperTrait> {
        self.polys.iter_mut().map(|it| -> &'a mut dyn CollidableWrapperTrait{it})
        .chain(self.lines.iter_mut().map(|it| -> &'a mut dyn CollidableWrapperTrait{it}))
        .chain(self.circles.iter_mut().map(|it| -> &'a mut dyn CollidableWrapperTrait{it}))
        .chain(self.points.iter_mut().map(|it| -> &'a mut dyn CollidableWrapperTrait{it}))    
    }

    fn get_collision_wrappers<'a>(&'a self) -> impl Iterator<Item = &'a dyn CollidableWrapperTrait> {
        self.polys.iter().map(|it| -> &'a dyn CollidableWrapperTrait{it})
        .chain(self.lines.iter().map(|it| -> &'a dyn CollidableWrapperTrait{it}))        
        .chain(self.circles.iter().map(|it| -> &'a dyn CollidableWrapperTrait{it}))        
        .chain(self.points.iter().map(|it| -> &'a dyn CollidableWrapperTrait{it}))        
    }

    fn get_total_collidables(&self) -> usize {
        self.get_collision_wrappers().count()
    }

    fn update_switches(&mut self) {
        self.game_input.right_switch.update_state(self.external_input.kbd.devices[0].right);
        self.game_input.left_switch.update_state(self.external_input.kbd.devices[0].left);        
    }

    fn update_object_controls(&mut self) {
        self.game_input.mov_horizontal = self.external_input.kbd.devices[0].d as isize - (self.external_input.kbd.devices[0].a as isize);
        self.game_input.mov_vertical = self.external_input.kbd.devices[0].w as isize - (self.external_input.kbd.devices[0].s as isize);
        self.game_input.rot = self.external_input.kbd.devices[0].e as isize - (self.external_input.kbd.devices[0].q as isize);
    }

    fn update_player_control(&mut self) {
        self.update_player_index();
        
        let player_index = self.player_index;

        for obj in self.get_collision_wrappers_mut() {
            let player_controlled_flag = obj.get_collidable_index() == player_index;
            obj.set_player_control(player_controlled_flag);
            if player_controlled_flag {
                obj.reset_collision_flag();
            }
        }
    }

    fn update_player_index(&mut self) {
        if self.player_index == 0 && self.game_input.left_switch.pressed() {
            self.player_index = self.get_total_collidables() - 1; 
            return;
        }
        if self.player_index == self.get_total_collidables() - 1 && self.game_input.right_switch.pressed() {
            self.player_index = 0;
            return;
        }
        self.player_index += self.game_input.right_switch.pressed() as usize;
        self.player_index -= self.game_input.left_switch.pressed() as usize;
    }

    fn update_colors(&mut self) {
        let player_index = self.player_index;

        for obj in self.get_collision_wrappers_mut() {
            let player_controlled_flag = obj.get_collidable_index() == player_index;
            match (player_controlled_flag, obj.has_collided_in_past()) {
                (true, _) => obj.set_color(Color::new(1.0, 0.2, 0.2, 1.0)),
                (false, false) => obj.set_color(Color::new(1.0, 1.0, 1.0, 1.0)),
                (false, true) => obj.set_color(Color::new(0.2, 0.2, 1.0, 1.0))
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