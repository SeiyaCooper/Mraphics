pub trait Material {
    fn identifier(&self) -> &'static str;
    fn shader_code(&self) -> String;
}
