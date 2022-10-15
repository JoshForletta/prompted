use std::fmt::Display;

pub struct NoComponent {}

impl Display for NoComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Component for NoComponent {}

pub trait Component: Display {

    fn include(&self) -> bool {
        false
    }
}
