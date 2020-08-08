use tetra::{
    Context,
    graphics::Texture,
    graphics::text::VectorFontBuilder,
    graphics::text::Font,
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

    pub fn get_font(ctx: &mut Context, file: &str, size: f32) -> Font {
        let file = Self::get(file).unwrap();

        // Need to leak the data because `from_file_data` is expecting
        // to get a static ref to data unlike it's `Texture` counterpart
        let file = Box::new(file);
        let file = Box::leak(file);

        VectorFontBuilder::from_file_data(file)
            .unwrap()
            .with_size(ctx, size)
            .unwrap()
    }
}
