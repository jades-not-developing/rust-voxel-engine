use crate::{loader::RawModel, texture::Texture};

#[derive(Clone)]
pub struct Model {
    pub data: RawModel,
    pub texture: Texture,
}