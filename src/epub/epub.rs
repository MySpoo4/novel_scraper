use std::path::PathBuf;

use crate::errors::{Error, Result};
use crate::models::{Chapter, MetaData};
use crowbook::{Book, Number};

pub struct Epub {
    book: Book,
}

impl Epub {
    pub fn new(meta_data: &MetaData) -> Self {
        let mut book = Book::new();
        book.set_options(&[
            ("title", meta_data.title.as_ref()),
            ("author", meta_data.author.as_ref()),
            ("lang", "en"),
        ]);
        Epub { book }
    }

    pub fn add_chapter(&mut self, chapter: Chapter) -> Result<()> {
        self.book
            .add_chapter_from_source(Number::Default, Self::build_md(&chapter).as_bytes(), false)
            .map(|book: &mut Book| {
                println!(
                    "Successfully added chapter #{}:\n{}\n",
                    book.chapters.len(),
                    &chapter.title
                )
            })
            .map_err(|_| Error::ChapterError)
    }

    pub fn build(&mut self, mut output_path: PathBuf) -> Result<()> {
        if output_path.extension().is_none() {
            let title = self
                .book
                .options
                .get_str("title")
                .unwrap_or("unspecified_title");
            output_path.push(title);
            output_path.set_extension("epub");
        }

        self.book
            .render_format_to_file("epub", output_path)
            .map(|_| ())
            .map_err(|_| Error::EpubBuildError)
    }

    fn build_md(chapter: &Chapter) -> String {
        format!("# {}\n {}", chapter.title, chapter.body)
    }
}
