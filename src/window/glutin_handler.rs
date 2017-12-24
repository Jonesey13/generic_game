use glium::glutin::Event;
use glium::Display;
use super::WindowHandler;
use glium::glutin::EventsLoop;
use glium::glutin::WindowEvent;

pub struct GlutinInput {
    focused_flag: bool
}

impl GlutinInput {
    pub fn new() -> GlutinInput {
        GlutinInput {
            focused_flag: true,
        }
    }
}

impl WindowHandler for GlutinInput {
    fn receive_input(&mut self, events_loop: &mut EventsLoop) {
        events_loop.poll_events( |event| {
            match event
            {
                Event::WindowEvent {
                    window_id: _,
                    event: WindowEvent::Focused(b),
                } => self.focused_flag = b,
                _ => (),
            }
        })
    }
    
    fn is_focused(&self) -> bool { self.focused_flag }
}
