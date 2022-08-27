#[derive(Clone, Copy)]
pub(crate) enum Texture {
    Lambert,
    Reflection(f64),
}
