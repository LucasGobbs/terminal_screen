pub trait BufferDrawer {
    fn add_glyphs(glyphs: Vec<char>);
    fn draw(buffer: Buffer);
}