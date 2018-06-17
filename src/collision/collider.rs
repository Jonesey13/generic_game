use collision::{CollisionObjectResults, Collidable, CollisionDetails, CollisionObjectState, 
                CollisionDataType, collision_logic, CollisionResults};
use debug::*;

pub struct Collider;

impl Collider {
    pub fn process_all<T: Clone + CollisionDataType> (mut collidables: Vec<&mut Collidable<Data=T>>) {
        loop {
            if let Some ((first_collidable, rest)) = collidables.split_last_mut() {
                for second_collidable in rest.into_iter() {
                    if T::has_exclusion_rules() {
                        let data1 = first_collidable.get_own_collision_data();
                        let data2 = second_collidable.get_own_collision_data();

                        if T::can_collide(&data1, &data2) {
                            if let Some((mut details1, mut details2)) = Collider::process_pair_of_collidables(*first_collidable, *second_collidable) {
                                first_collidable.add_collision_results(CollisionResults::new(details1, data2));
                                second_collidable.add_collision_results(CollisionResults::new(details2, data1));
                            }
                        }
                    } else {
                        if let Some((mut details1, mut details2)) = Collider::process_pair_of_collidables(*first_collidable, *second_collidable) {
                            let data1 = first_collidable.get_own_collision_data();
                            let data2 = second_collidable.get_own_collision_data();

                            first_collidable.add_collision_results(CollisionResults::new(details1, data2));
                            second_collidable.add_collision_results(CollisionResults::new(details2, data1));
                        }
                    }
                }
            }
            else {
                break;
            }
            collidables.pop();
        }
    }

    fn process_pair_of_collidables<T: Clone> (first: &Collidable<Data=T>, second: &Collidable<Data=T>) 
        -> Option<(CollisionDetails, CollisionDetails)> {

        let mut results: Option<(CollisionObjectResults, CollisionObjectResults)> = None;
        let (mut location1, mut location2) = (0, 0);
        
        for (obj_loc1, first_obj) in first.get_collision_objects().iter().enumerate() {
            for (obj_loc2, second_obj) in second.get_collision_objects().iter().enumerate() {
                if let Some((obj_results1, obj_results2))
                    = Collider::process_pair_of_object_states(first_obj, second_obj) {

                    if let Some(old_time) = results.clone().and_then(|res| {Some(res.0.time)}) {
                        if obj_results1.time > old_time {
                            continue
                        }
                    }
                    results = Some((obj_results1, obj_results2));
                    location1 = obj_loc1;
                    location2 = obj_loc2;
                }
            }
        }

        if let Some((results1, results2)) = results {
            Some((CollisionDetails::new(location1, results1.details, results1.time)
                , CollisionDetails::new(location2, results2.details, results2.time)))
        } else {
            None
        }
    }

    fn process_pair_of_object_states (first: &CollisionObjectState, second: &CollisionObjectState) 
        -> Option<(CollisionObjectResults, CollisionObjectResults)> {
        match (first, second) {
            // Reflexive (points can't collide with points)
            (&CollisionObjectState::Circ(ref n1, ref p1), &CollisionObjectState::Circ(ref n2, ref p2)) 
                => collision_logic::circ_circ_coll(&n1, &p1, &n2, &p2),
            (&CollisionObjectState::ConPoly(ref n1, ref p1), &CollisionObjectState::ConPoly(ref n2, ref p2)) 
                => collision_logic::poly_poly_coll(n1, p1, n2, p2),
            (&CollisionObjectState::Line(ref n1, ref p1), &CollisionObjectState::Line(ref n2, ref p2))
                => {let res = collision_logic::poly_poly_coll(n1, p1, n2, p2); 
                res.and_then(|(res0, res1)| {Some((res0.to_line_results(), res1.to_line_results()))})},

            // Symmetric
            (&CollisionObjectState::Circ(ref n1, ref p1), &CollisionObjectState::ConPoly(ref n2, ref p2)) 
                => collision_logic::circ_poly_coll(&n1, &p1, n2, p2),
            (&CollisionObjectState::ConPoly(ref n1, ref p1), &CollisionObjectState::Circ(ref n2, ref p2)) 
                => {let res = collision_logic::circ_poly_coll(&n2, &p2, n1, p1); 
                res.and_then(|(res0, res1)|{Some((res1, res0))})},

            (&CollisionObjectState::Line(ref n1, ref p1), &CollisionObjectState::ConPoly(ref n2, ref p2)) 
                => {let res = collision_logic::poly_poly_coll(n2, p2, n1, p1); 
                res.and_then(|(res0, res1)| {Some((res1.to_line_results(), res0))})},
            (&CollisionObjectState::ConPoly(ref n1, ref p1), &CollisionObjectState::Line(ref n2, ref p2)) 
                => {let res = collision_logic::poly_poly_coll(n1, p1, n2, p2); 
                res.and_then(|(res0, res1)| {Some((res0, res1.to_line_results()))})},

            (&CollisionObjectState::Point(n1, p1), &CollisionObjectState::ConPoly(ref n2, ref p2)) 
                => {let res = collision_logic::poly_point_coll(n2, p2, n1, p1); 
                res.and_then(|(res0, res1)|{Some((res1, res0))})},
            (&CollisionObjectState::ConPoly(ref n1, ref p1), &CollisionObjectState::Point(n2, p2)) 
                => collision_logic::poly_point_coll(n1, p1, n2, p2),

            (&CollisionObjectState::Circ(ref n1, ref p1), &CollisionObjectState::Line(ref n2, ref p2)) 
                => {let res = collision_logic::circ_poly_coll(&n1, &p1, n2, p2); 
                res.and_then(|(res0, res1)| {Some((res0, res1.to_line_results()))})},
            (&CollisionObjectState::Line(ref n1, ref p1), &CollisionObjectState::Circ(ref n2, ref p2)) 
                => {let res = collision_logic::circ_poly_coll(&n2, &p2, n1, p1); 
                res.and_then(|(res0, res1)| {Some((res1.to_line_results(), res0))})},
            
            (&CollisionObjectState::Line(ref n1, ref p1), &CollisionObjectState::Point(n2, p2)) 
                => {let res = collision_logic::poly_point_coll(n1, p1, n2, p2); 
                res.and_then(|(res0, res1)| {Some((res0.to_line_results(), res1))})},
            (&CollisionObjectState::Point(n1, p1), &CollisionObjectState::Line(ref n2, ref p2)) 
                => {let res = collision_logic::poly_point_coll(n2, p2, n1, p1); 
                res.and_then(|(res0, res1)| {Some((res1, res0.to_line_results()))})},
            
            (&CollisionObjectState::Circ(ref n1, ref p1), &CollisionObjectState::Point(n2, p2)) 
                => collision_logic::circ_point_coll(&n1, &p1, n2, p2),
            (&CollisionObjectState::Point(n1, p1), &CollisionObjectState::Circ(ref n2, ref p2)) 
                => {let res = collision_logic::circ_point_coll(&n2, &p2, n1, p1); 
                res.and_then(|(res0, res1)| {Some((res1, res0))})},
            
            _ => None,
        }
    }
}