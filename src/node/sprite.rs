pub enum Texture {
    ByteTexture(Vec<u8>),
}

pub struct Sprite {
    texture: Texture
}
