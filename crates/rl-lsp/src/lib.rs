//! LSP server for rl, built on [`tower_lsp`].
//!
//! Communicates with editors over stdio using JSON-RPC.
//! Currently supports:
//! - **Diagnostics** - lex/parse/type-check errors on every file change
//! - **Hover** - markdown type info at the cursor position
//!
//! # Architecture
//!
//! ```text
//! editor (VS Code, Zed, Neovim, ...)
//!     │  JSON-RPC over stdio
//! |---V----------------------------|
//! |  Backend  (tower-lsp handler)  |
//! |  |-- did_open / did_change --> pipeline::run_pipeline  --> diagnostics
//! |  |-- hover -----------------> hover::run_hover         --> hover info
//! |--------------------------------|
//! ```

use tower_lsp::{LspService, Server};

use crate::backend::Backend;

mod backend;
mod goto_definition;
mod hover;
mod pipeline;
mod refernces;
mod to_diagnostic;
mod utils;

/// Starts the LSP server, reading JSON-RPC from stdin and writing to stdout.
///
/// Blocks indefinitely - intended to be spawned as a long-running process
/// by the editor's LSP client.
pub async fn run_lsp() {
    // declare stdin and out for editor and lsp communications
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    // creating a new lsp service by wrapping the backend struct in a service
    // that handles the json-rpc protocol
    // json-rpc uses methods like didopen and if the method matches
    // the implemented ones it returns a json response to the editor
    // with the diagnostic data
    let (service, socket) = LspService::new(|client| Backend {
        client,
        docs: Default::default(),
    });

    // creates a new server and passes the stdin stdout and the socket
    // and serves the lsp services
    // actually this should be blocked to work infinitly
    Server::new(stdin, stdout, socket).serve(service).await;
}
