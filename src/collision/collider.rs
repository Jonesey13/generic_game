use super::{CollisionObjectResults, Collidable, CollisionObjectState, collision_logic, CollisionResults};

pub struct Collider;

impl Collider {
    pub fn process_all<T: Clone> (mut collidables: Vec<&mut Collidable<Data=T>>) {
        loop {
            if let Some ((first_collidable, rest)) = collidables.split_last_mut() {
                for second_collidable in rest.into_iter() {
                    let (mut results1, mut results2) = Collider::process_pair_of_collidables(*first_collidable, *second_collidable);

                    if results1.collided || results2.collided {
                        let data1 = first_collidable.get_own_collision_data();
                        let data2 = second_collidable.get_own_collision_data();

                        if results1.collided {
                            results1.data = Some(data2);
                            first_collidable.add_collision_results(results1);
                        }

                        if results2.collided {
                            results2.data = Some(data1);
                            second_collidable.add_collision_results(results2);
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
        -> (CollisionResults<T>, CollisionResults<T>) {

        let (mut results1, mut results2) = (CollisionObjectResults::no_collision(), CollisionObjectResults::no_collision());
        let (mut location1, mut location2) = (0, 0);
        
        for (obj_loc1, first_obj) in first.get_collision_objects().iter().enumerate() {
            for (obj_loc2, second_obj) in second.get_collision_objects().iter().enumerate() {
                let (obj_results1, obj_results2): (CollisionObjectResults<T>, CollisionObjectResults<T>) 
                    = Collider::process_pair_of_object_states(first_obj, second_obj);

                if let Some(new_time) = obj_results1.time {
                    if let Some(old_time) = results1.time {
                        if new_time > old_time {
                            continue
                        }
                    }
                    results1 = obj_results1;
                    results2 = obj_results2;
                    location1 = obj_loc1;
                    location2 = obj_loc2;
                }
            }
        }

        (CollisionResults::new_with_location(location1, results1), CollisionResults::new_with_location(location2,results2))
    }

    fn process_pair_of_object_states<T: Clone> (first: &CollisionObjectState, second: &CollisionObjectState) 
        -> (CollisionObjectResults<T>, CollisionObjectResults<T>) {
        match (first, second) {
            // Reflexive (points can't collide with points)
            (&CollisionObjectState::Circ(ref n1, ref p1), &CollisionObjectState::Circ(ref n2, ref p2)) 
                => collision_logic::circ_circ_coll(&n1, &p1, &n2, &p2),
            (&CollisionObjectState::ConPoly(ref n1, ref p1), &CollisionObjectState::ConPoly(ref n2, ref p2)) 
                => collision_logic::poly_poly_coll(n1, p1, n2, p2),
            (&CollisionObjectState::Line(ref n1, ref p1), &CollisionObjectState::Line(ref n2, ref p2))
                => {let res = collision_logic::poly_poly_coll(n1, p1, n2, p2); (res.0.to_line_results(), res.1.to_line_results())},

            // Symmetric
            (&CollisionObjectState::Circ(ref n1, ref p1), &CollisionObjectState::ConPoly(ref n2, ref p2)) 
                => collision_logic::circ_poly_coll(&n1, &p1, n2, p2),
            (&CollisionObjectState::ConPoly(ref n1, ref p1), &CollisionObjectState::Circ(ref n2, ref p2)) 
                => {let res = collision_logic::circ_poly_coll(&n2, &p2, n1, p1); (res.1, res.0)},

            (&CollisionObjectState::Line(ref n1, ref p1), &CollisionObjectState::ConPoly(ref n2, ref p2)) 
                => {let res = collision_logic::poly_poly_coll(n2, p2, n1, p1); (res.1.to_line_results(), res.0)},
            (&CollisionObjectState::ConPoly(ref n1, ref p1), &CollisionObjectState::Line(ref n2, ref p2)) 
                => {let res = collision_logic::poly_poly_coll(n1, p1, n2, p2); (res.0, res.1.to_line_results())},

            (&CollisionObjectState::Point(ref n1, ref p1), &CollisionObjectState::ConPoly(ref n2, ref p2)) 
                => {let res = collision_logic::poly_point_coll(n2, p2, &n1, &p1); (res.1, res.0)},
            (&CollisionObjectState::ConPoly(ref n1, ref p1), &CollisionObjectState::Point(ref n2, ref p2)) 
                => collision_logic::poly_point_coll(n1, p1, &n2, &p2),

            (&CollisionObjectState::Circ(ref n1, ref p1), &CollisionObjectState::Line(ref n2, ref p2)) 
                => {let res = collision_logic::circ_poly_coll(&n1, &p1, n2, p2); (res.0, res.1.to_line_results())},
            (&CollisionObjectState::Line(ref n1, ref p1), &CollisionObjectState::Circ(ref n2, ref p2)) 
                => {let res = collision_logic::circ_poly_coll(&n2, &p2, n1, p1); (res.1.to_line_results(), res.0)},
            
            (&CollisionObjectState::Line(ref n1, ref p1), &CollisionObjectState::Point(ref n2, ref p2)) 
                => {let res = collision_logic::poly_point_coll(n1, p1, n2, p2); (res.0.to_line_results(), res.1)},
            (&CollisionObjectState::Point(ref n1, ref p1), &CollisionObjectState::Line(ref n2, ref p2)) 
                => {let res = collision_logic::poly_point_coll(n2, p2, n1, p1); (res.1, res.0.to_line_results())},
            
            (&CollisionObjectState::Circ(ref n1, ref p1), &CollisionObjectState::Point(ref n2, ref p2)) 
                => collision_logic::circ_point_coll(&n1, &p1, n2, p2),
            (&CollisionObjectState::Point(ref n1, ref p1), &CollisionObjectState::Circ(ref n2, ref p2)) 
                => {let res = collision_logic::circ_point_coll(&n2, &p2, n1, p1); (res.1, res.0)},
            
            _ => (CollisionObjectResults::no_collision(), CollisionObjectResults::no_collision()),
        }
    }
}