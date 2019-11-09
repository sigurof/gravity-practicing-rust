use kiss3d::window::Window;

pub trait GameObject {
    fn update(&mut self, window: &mut Window);
}
