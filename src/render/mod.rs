pub mod template;
pub mod json;

use anyhow::Result;
use serde::Serialize;

use crate::ProgramMeta;
use crate::book::Song;
use crate::project::{Metadata, Output, Project};

#[derive(Serialize, Debug)]
pub struct RenderContext<'a> {
    book: &'a Metadata,
    songs: &'a [Song],
    output: &'a Metadata,
    program: ProgramMeta,
}

pub trait Render {
    fn render<'a>(project: &'a Project, output: &'a Output) -> Result<&'a Output>;
}

pub use self::template::{DefaultTemaplate, RHtml, RTex, RHovorka};
pub use self::json::RJson;
