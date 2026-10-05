use super::fonts::FONTS;
use chrono::{Datelike, FixedOffset, Utc};
use chrono_tz::Europe::Helsinki;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime, Dict, Duration};
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};

static TEMPLATE: &str = include_str!("../../templates/pdf/contract.typ");

#[derive(Debug, Clone)]
pub struct TypstWorld {
    source: Source,
    library: LazyHash<Library>,
    main_id: FileId,
}

impl TypstWorld {
    pub fn new(inputs: Dict) -> Self {
        let main_id = RootedPath::new(
            VirtualRoot::Project,
            VirtualPath::new("main.typ").expect("static path is valid"),
        )
        .intern();
        Self {
            library: LazyHash::new(Library::builder().with_inputs(inputs).build()),
            main_id,
            source: Source::new(main_id, TEMPLATE.to_owned()),
        }
    }
}

impl World for TypstWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &FONTS.book
    }

    fn main(&self) -> FileId {
        self.main_id
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main_id {
            Ok(self.source.clone())
        } else {
            Err(FileError::NotFound(id.vpath().get_without_slash().into()))
        }
    }

    // We have no files
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound(id.vpath().get_without_slash().into()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        FONTS.list.get(index).cloned()
    }

    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        let date = match offset {
            None => Utc::now().with_timezone(&Helsinki).fixed_offset(),
            Some(duration) => {
                #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
                let fixed_offset = FixedOffset::east_opt(duration.seconds() as i32)?;
                Utc::now().with_timezone(&fixed_offset)
            }
        };
        Datetime::from_ymd(
            date.year(),
            u8::try_from(date.month()).ok()?,
            u8::try_from(date.day()).ok()?,
        )
    }
}
