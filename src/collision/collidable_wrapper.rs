use crate::collision::*;
use crate::geometry::*;
use crate::rendering::*;

#[derive(Clone)]
pub struct CollidableWrapper<C: ToCollisionObjects + Clone, D: Clone + CollisionDataType> {
    collidable: C,
    coll_results: Option<CollisionResults<D>>,
    collidable_index: usize,
    collidable_prev: Option<C>,
    data: D,
    player_controlled: bool,
    color: Color,
    has_collided_in_past: bool,
    last_collision_details: Option<CollisionDetails>
}

impl<C: ToCollisionObjects + Clone, D: Clone + CollisionDataType> CollidableWrapper<C, D> {
    pub fn new(collidable: C, collidable_index: usize, data: D) -> Self {
        CollidableWrapper {
            collidable,
            coll_results: None,
            collidable_index,
            collidable_prev: None,
            data,
            player_controlled: false,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            has_collided_in_past: false,
            last_collision_details: None,
        }
    }

    pub fn coll_results_color() -> Color {
        Color::new(0.0, 1.0, 0.0, 1.0)
    }
}

pub trait CollidableWrapperTrait: TwoDTransformable {
    fn render(&self, depth: f64) -> Vec<Box<StandardRenderable>>;
    fn set_prev(&mut self);
    fn set_player_control(&mut self, flag: bool);
    fn is_player_controlled(&self) -> bool;
    fn get_collidable_index(&self) -> usize;
    fn get_color(&self) -> Color;
    fn set_color(&mut self, color: Color);
    fn has_collided_in_past(&self) -> bool;
    fn reset_collision_flag(&mut self);
    fn render_coll_results(&self, depth: f64) -> Vec<Box<StandardRenderable>>;
}

impl<C: Clone + ToCollisionObjects + TwoDTransformable, D: Clone + CollisionDataType> CollidableWrapperTrait for CollidableWrapper<C, D>  {
    fn render(&self, depth: f64) -> Vec<Box<StandardRenderable>> {
        self.collidable.to_collision_objects().iter().flat_map(|obj| {obj.render(self.get_color(), depth, false)}).collect()
    }

    fn render_coll_results(&self, depth: f64) -> Vec<Box<StandardRenderable>> {
        if self.has_collided_in_past {
            let collision_object_details = self.last_collision_details.clone().unwrap().object_details;
            let collision_location = self.last_collision_details.clone().unwrap().location;

            self.collidable
            .to_collision_objects()[collision_location]
            .render_collision_details(
                collision_object_details, 
                CollidableWrapper::<C,D>::coll_results_color(), 
                depth, 
                false)
        }
        else {
            vec![]
        }
    }

    fn set_prev(&mut self) {
        self.collidable_prev = Some(self.collidable.clone());
    }

    fn set_player_control(&mut self, flag: bool) {
        self.player_controlled = flag;
    }

    fn is_player_controlled(&self) -> bool {
        self.player_controlled
    }

    fn get_collidable_index(&self) -> usize {
        self.collidable_index
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn has_collided_in_past(&self) -> bool {
        self.has_collided_in_past
    }

    fn reset_collision_flag(&mut self) {
        self.has_collided_in_past = false;
        self.last_collision_details = None;
    }
}

impl<C: Clone + ToCollisionObjects + TwoDTransformable, D: Clone + CollisionDataType> Collidable for CollidableWrapper<C, D> {
    type Data = D;

    fn get_collision_objects(&self) -> Vec<CollisionObjectState> {
        if let Some(coll_prev) = self.collidable_prev.clone() {
            return self.collidable
            .to_collision_objects().into_iter()
            .zip(coll_prev.to_collision_objects().into_iter())
            .map(|(current, prev)| {current.build_state(prev)})
            .collect()
        }
        vec![]
    }

    fn get_earliest_collision_results(&self) -> Option<CollisionResults<Self::Data>> {
        self.coll_results.clone().into()
    }

    fn add_collision_results(&mut self, new_results: CollisionResults<Self::Data>) {
        self.coll_results = new_results.clone().into();
        self.has_collided_in_past = true;
        self.last_collision_details = Some(new_results.details);
    }

    fn get_own_collision_data(&self) -> Self::Data { self.data.clone() }
}

impl<C: Clone + ToCollisionObjects + TwoDTransformable, D: Clone + CollisionDataType> TwoDTransformable for CollidableWrapper<C, D> {
    fn shift_by(&mut self, shift: Point) {
        self.collidable.shift_by(shift);
    }

    fn rotate_at_center(&mut self, rot_angle: f64) {
        self.collidable.rotate_at_center(rot_angle);
    }

    fn rotate_at_origin(&mut self, rot_angle: f64) {
        self.collidable.rotate_at_origin(rot_angle);
    }

    fn get_center(&self) -> Point {
        self.collidable.get_center()
    }

    fn scale_by(&mut self, scale_factor: f64) {
        self.collidable.scale_by(scale_factor)
    }
}
