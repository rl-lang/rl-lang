mod backend;
mod pipeline;
mod to_diagnostic;
mod utils;

use tower_lsp::{LspService, Server};

use crate::lsp::backend::Backend;

pub async fn run_lsp() {
    // declare stdin and out for editor and lsp communications
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    // creating a new lsp service by wrapping the backend struct in a service
    // that handles the json-rpc protocol
    // json-rpc uses methods like didopen and if the method matches
    // the implemented ones it returns a json response to the editor
    // with the diagnostic data
    let (service, socket) = LspService::new(|client| Backend { client });

    // creates a new server and passes the stdin stdout and the socket
    // and serves the lsp services
    // actually this should be blocked to work infinitly
    Server::new(stdin, stdout, socket).serve(service).await;
}
