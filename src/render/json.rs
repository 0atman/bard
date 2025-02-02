use std::fs::File;

use crate::project::{Project, Output};
use crate::PROGRAM_META;
use crate::error::*;
use super::{Render, RenderContext};

pub struct RJson;

impl Render for RJson {
    fn render<'a>(project: &'a Project, output: &'a Output) -> Result<&'a Output> {
        let context = RenderContext {
            book: project.metadata(),
            songs: project.songs(),
            output: &output.metadata,
            program: PROGRAM_META,
        };

        let path = &output.file;

        File::create(&path)
            .map_err(Error::from)
            .and_then(|mut f| serde_json::to_writer_pretty(&mut f, &context).map_err(Error::from))
            .with_context(|| format!("Error writing output file: `{}`", path.display()))?;

        Ok(output)
    }
}
