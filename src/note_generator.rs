use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use markdown::{mdast::Node, Constructs, Options, ParseOptions};

#[derive(Debug)]
pub struct NoteGenerator {
    notes: HashMap<PathBuf, Node>,
}

impl NoteGenerator {
    pub fn new() -> NoteGenerator {
        Self {
            notes: HashMap::new(),
        }
    }

    pub fn create_note(&mut self, src: &Path, target: &Path) -> eyre::Result<()> {
        let src_text = fs::read_to_string(src)?;
        let mdast = match markdown::to_mdast(
            &src_text,
            &ParseOptions {
                constructs: Constructs {
                    frontmatter: true,
                    ..Constructs::default()
                },
                ..ParseOptions::default()
            },
        ) {
            Ok(mdast) => mdast,
            Err(msg) => {
                return Err(eyre::eyre!(
                    "Markdown error in '{}': {:?}",
                    src.display(),
                    msg
                ))
            }
        };

        let note_path = target.with_extension("html");
        println!("{:#?}", mdast);
        self.notes.insert(note_path, mdast);
        let note_html = markdown::to_html_with_options(
            &src_text,
            &Options {
                parse: ParseOptions {
                    constructs: Constructs {
                        frontmatter: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Options::default()
            },
        )
        .unwrap();
        dbg!(&note_html);
        Ok(())
    }
}
