use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../frontend/dist_new"]
pub struct Assets;
