use gg::games::Game;
use gg::games::GameInput;
use gg::games::view_details::{ViewDetails2D, ViewDetails};
use gg::input::keyboard::KeyboardInput;
use gg::rendering::*;
use gg::geometry::*;

#[allow(dead_code)]
#[derive(Default, Clone)]
pub struct RenderableTestGame {
    view_details: ViewDetails2D,
    user_input: UserInput,
    external_input: ExternalInput
}

impl Game for RenderableTestGame {
    type Primitive = StandardPrimitive;

    fn update_input(&mut self) {
        self.user_input.right_left = self.external_input.kbd.get_d() as isize - (self.external_input.kbd.get_a() as isize);
        self.user_input.up_down = self.external_input.kbd.get_w() as isize - (self.external_input.kbd.get_s() as isize);
        self.user_input.in_out = self.external_input.kbd.get_r() as isize - (self.external_input.kbd.get_f() as isize);
        self.user_input.anticlockwise_clockwise = self.external_input.kbd.get_e() as isize - (self.external_input.kbd.get_q() as isize);
    }

    fn update_logic(&mut self, t_step: f64) {
        self.view_details.camera_pos.x = self.view_details.camera_pos.x + (self.user_input.right_left as f64) * t_step;
        self.view_details.camera_pos.y = self.view_details.camera_pos.y + (self.user_input.up_down as f64) * t_step;
        self.view_details.viewport_height = self.view_details.viewport_height + (self.user_input.in_out as f64) * t_step;
        self.view_details.viewport_length = self.view_details.viewport_length + (self.user_input.in_out as f64) * t_step;
        let current_rotation = self.view_details.get_rotation_angle();
        let new_rotation = current_rotation + (self.user_input.anticlockwise_clockwise as f64) * t_step;
        self.view_details.set_rotation_angle(new_rotation);
    }

    fn get_view(&self) -> ViewDetails {
        ViewDetails::TwoDim(self.view_details.clone())
    }
    
    fn get_renderables(&mut self, _: WindowSpec) -> Vec<Box<StandardRenderable>> {
        // let rect = Rectangle {
        //     length: 1.0,
        //     height: 1.0,
        //     rot: Rotation::new(0.0),
        //     pos: Point3::new(0.0, 0.0, 0.1),
        //     color: Color::new(0.0, 1.0, 0.0, 1.0)
        // };
        let _circ = CircleRenderable {
            radius: 0.7,
            pos: Point3::new(-0.0, 0.0, 0.1),
            color: Color::new(1.0, 0.0, 0.0, 1.0),
            fixed: true
        };

        let _ann = Annulus {
            radial_dim: Point::new(0.4, 0.5),
            pos: Point3::new(0.2, -0.3, -0.1),
            color: Color::new(0.0, 0.0, 1.0, 1.0),
            fixed: false
        };

        let _text = PlainText {
            content: "llllllll there! |".to_string(),
            position: Point3::new(0.0, 0.0, 0.0),
            scale: Point::new(0.2, 0.2),
            transform: Rotation::new(0.0).get_matrix(),
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            fixed: false,
            align: TextAlign::Centered
        };
        
        let poly_corners = vec![
            Point::new(0.5, 0.5),
            Point::new(0.0, 0.5),
            Point::new(0.0, 0.0),
            Point::new(-0.2, 0.0),
            Point::new(-0.2, -0.2),
            Point::new(0.2, -0.2)
        ];
        let _poly = Polygon::new_regular(poly_corners, Point::zero(), Point3::zero(), Color::new(1.0, 0.0, 0.0, 1.0), false);

        let _line = LineRenderable::new_rounded(
            Point::new(-0.5, -0.5),
            Point::new(0.5, -0.25),
            0.05,
            Color::new(0.0, 0.5, 0.0, 1.0),
            0.0,
            false
        );

        let _arrow = Arrow::new_rounded(
            Point::new(0.5, 0.0),
            Point::new(-0.5, 0.0),
            0.05,
            Point::new(0.2, 0.2),
            Color::new(0.5, 0.5, 1.0, 1.0),
            0.0,
            false
        );

        let tex_rect1 = TextureRect::new_regular(
            0.5, 
            0.5, 
            Point3::new(0.5, 0.5, 0.0), 
            Point3::new(0.0, 0.0, 0.0),
            Point::new(1.0, 1.0),
            false
        );

        let tex_rect2 = TextureRect::new_regular(
            0.5, 
            0.5, 
            Point3::new(-0.5, -0.5, 0.0), 
            Point3::new(0.0, 0.0, 1.0),
            Point::new(0.5, 0.5),
            false
        );

        let _box_border_fixed = BoxBorder::new_rounded(0.01, 0.1, Point3::new(0.0, 0.0, -0.2), 0.5, 0.5, Color::new(1.0, 1.0, 0.0, 1.0), true);
        let _box_border = BoxBorder::new_rounded(0.01, 0.1, Point3::new(0.0, 0.0, -0.2), 0.5, 0.5, Color::new(1.0, 1.0, 0.0, 1.0), false);        

        vec![
            Box::new(tex_rect1),
            Box::new(tex_rect2)
        ]
    }

    fn get_input<'a>(&'a mut self) -> Option <&'a mut GameInput> {
        Some(&mut self.external_input)
    }
}

#[derive(Clone, Default)]
struct UserInput {
    right_left: isize,
    up_down: isize,
    in_out: isize,
    anticlockwise_clockwise: isize
}

#[derive(Clone, Default)]
struct ExternalInput {
    kbd: KeyboardInput,
}

impl GameInput for ExternalInput {
    fn get_kbd_inp<'a>(&'a mut self) -> Option<&'a mut KeyboardInput> { Some(&mut self.kbd) }
}
