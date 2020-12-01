use futures::future;
use futures::stream::{TryStream, TryStreamExt};
use inotify::{Event, EventMask, Inotify, WatchMask};
use snafu::{futures::try_stream::TryStreamExt as SnafuTSE, Backtrace, ResultExt, Snafu};
use std::io::{self, BufReader, Read};
use std::path::PathBuf;
use uuid::Uuid;

use crate::api::model;
use crate::state::State;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("INotify Error: {}", source))]
    #[snafu(visibility(pub))]
    INotifyError {
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("File Error '{}': {}", path.display(), source))]
    #[snafu(visibility(pub))]
    FileIOError {
        path: PathBuf,
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("File Error: {}", details))]
    #[snafu(visibility(pub))]
    FileError { details: String },

    #[snafu(display("UUID Error: {}", source))]
    #[snafu(visibility(pub))]
    UuidError { source: uuid::Error },

    #[snafu(display("YAML Error: {}", source))]
    #[snafu(visibility(pub))]
    YamlError { source: serde_yaml::Error },
}

pub struct Watcher {
    path: PathBuf,
    buffer: [u8; 4096],
}

impl Watcher {
    pub fn new(path: PathBuf) -> Self {
        Watcher {
            path,
            buffer: [0u8; 4096],
        }
    }

    pub fn doc_stream(
        &mut self,
        state: State,
    ) -> Result<impl TryStream<Ok = model::DocSpec, Error = Error> + '_, io::Error> {
        let mut inotify = Inotify::init()?;

        inotify.add_watch(
            self.path.clone(),
            WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE,
        )?;

        let event_stream = inotify.event_stream(&mut self.buffer[..])?;

        let s1 = state.clone();
        let s2 = state;
        Ok(event_stream
            .context(INotifyError)
            .and_then(move |event| event_to_path(event, s1.clone()))
            .try_filter_map(future::ok)
            .and_then(move |path| path_to_doc_spec(path, s2.clone()))
            .try_filter_map(future::ok))
    }
}

fn event_to_path(
    event: Event<std::ffi::OsString>,
    _state: State,
) -> impl future::TryFuture<Ok = Option<PathBuf>, Error = Error> {
    let opt_path = match event.name {
        Some(name) => {
            let path = PathBuf::from(name);
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    if event.mask.contains(EventMask::CREATE) {
                        if event.mask.contains(EventMask::ISDIR) {
                            // println!("Directory created: {:?}", name);
                            None
                        } else {
                            Some(path)
                        }
                    } else if event.mask.contains(EventMask::DELETE) {
                        None
                    } else if event.mask.contains(EventMask::MODIFY) {
                        if event.mask.contains(EventMask::ISDIR) {
                            // println!("Directory modified: {:?}", path);
                            None
                        } else {
                            Some(path)
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        None => None,
    };
    future::ok(opt_path)
}

fn path_to_doc_spec(
    path: PathBuf,
    state: State,
) -> impl future::TryFuture<Ok = Option<model::DocSpec>, Error = Error> {
    // FIXME: Hardcoded base dir
    let mut p = state.settings.service.path;
    p.push(path.clone());

    let res = std::fs::File::open(p.clone())
        .context(FileIOError { path: p.clone() })
        .and_then(|file| {
            let mut reader = BufReader::new(file);
            let mut buffer = String::new();
            reader
                .read_to_string(&mut buffer)
                .context(FileIOError { path: p })?;
            Ok(buffer)
        })
        .and_then(|contents| {
            // This condition occurs when working with eg neovim...
            // I'm not trying to investigate, just guarding against it.
            if contents.is_empty() {
                return Ok(None);
            }
            let v: Vec<&str> = contents.splitn(3, "---").collect();
            if v.len() < 3 {
                return Err(Error::FileError {
                    details: format!("content length: {}", contents.len()),
                });
            }
            let base = path
                .file_stem()
                .ok_or(snafu::NoneError)
                .context(FileError {
                    details: String::from("Invalid Stem"),
                })?
                .to_str()
                .ok_or(snafu::NoneError)
                .context(FileError {
                    details: String::from("Invalid Filename UTF8 Conversion"),
                })?;

            let id = Uuid::parse_str(base).context(UuidError)?;

            let front: model::Front = serde_yaml::from_str(v[1]).context(YamlError)?;

            Ok(Some(model::DocSpec {
                id,
                title: front.title,
                outline: front.outline,
                author_fullname: front.author.fullname,
                author_resource: front.author.resource,
                tags: front.tags,
                image_title: front.image.title,
                image_resource: front.image.resource,
                image_author_fullname: front.image.author.fullname,
                image_author_resource: front.image.author.resource,
                kind: front.kind,
                genre: front.genre,
                content: String::from(v[2]),
            }))
        });

    future::ready(res)
}
