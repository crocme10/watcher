use clap::ArgMatches;
use futures::stream::TryStreamExt;
use futures::TryFutureExt;
use slog::{debug, error, info, Logger};
// use snafu::futures::try_future::TryFutureExt;
use snafu::ResultExt;

use watcher::error;
use watcher::settings::Settings;
use watcher::state::State;
use watcher::utils;
use watcher::watcher::Watcher;

#[allow(clippy::needless_lifetimes)]
pub async fn run<'a>(matches: &ArgMatches<'a>, logger: Logger) -> Result<(), error::Error> {
    let settings = Settings::new(matches)?;
    let state = State::new(&settings, &logger).await?;
    run_server(state).await
}

pub async fn run_server(state: State) -> Result<(), error::Error> {
    // We keep a copy of the logger before the context takes ownership of it.
    info!(state.logger, "Serving watcher");
    let mut watcher = Watcher::new(state.settings.service.path.clone());

    let journal_url = format!(
        "http://{}:{}/graphql",
        &state.settings.journal.host, &state.settings.journal.port
    );

    info!(state.logger, "GraphQL Target: {}", journal_url);

    let s1 = state.clone();
    if let Ok(mut stream) = watcher.doc_stream(s1).context(error::IOError {
        msg: String::from("Could not get doc stream"),
    }) {
        debug!(state.logger, "Document Stream available");

        loop {
            match stream.try_next().await {
                Ok(opt_doc) => {
                    debug!(state.logger, "event: document");
                    let data = format!(
                        r#"{{ "query": "{query}", "variables": {{ "doc": {{ "doc": {doc} }} }} }}"#,
                        query = "mutation CreateOrUpdateDocument($doc: DocumentRequestBody!) { createOrUpdateDocument(doc: $doc) { doc { id, front { title } } } }",
                        doc = serde_json::to_string(&opt_doc.unwrap()).unwrap() // FIXME
                    );
                    println!("request: {}", data);
                    let client = reqwest::Client::new();
                    client
                        .post(&journal_url)
                        .headers(utils::construct_headers())
                        .body(data)
                        .send()
                        .and_then(|resp| resp.text())
                        .map_ok_or_else(
                            |err| {
                                println!("error: {}", err);
                            },
                            |txt| {
                                println!("response: {}", txt);
                            },
                        )
                        .await
                }
                Err(err) => {
                    error!(state.logger, "Document Stream Error: {}", err);
                }
            }
        }
    } else {
        error!(state.logger, "document stream error");
    }
    drop(watcher);
    info!(state.logger, "Terminating Watcher");

    Ok(())
}
