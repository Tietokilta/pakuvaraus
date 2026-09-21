use std::sync::LazyLock;
use typst::foundations::Bytes;
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;

pub static FONTS: LazyLock<Fonts> = LazyLock::new(Fonts::load);

pub struct Fonts {
    pub book: LazyHash<FontBook>,
    pub list: Vec<Font>,
}

impl Fonts {
    fn load() -> Self {
        let list: Vec<Font> = typst_assets::fonts()
            .flat_map(|data| Font::iter(Bytes::new(data)))
            .collect();
        let book = LazyHash::new(FontBook::from_fonts(&list));
        Self { book, list }
    }
}
