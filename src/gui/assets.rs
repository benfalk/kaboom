use tetra::{
    Context,
    graphics::Texture,
};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources/"]
pub struct Assets;

impl Assets {
    pub fn get_texture(ctx: &mut Context, file: &str) -> Texture {
        let file = Self::get(file).unwrap();
        Texture::from_file_data(ctx, file.as_ref()).unwrap()
    }
}
