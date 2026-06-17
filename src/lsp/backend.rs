use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::lsp::pipeline::run_pipeline;

pub struct Backend {
    pub client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    // can be considered a handshake the TextDocumentSyncKind::FULL is asking
    // the current editor to send the file contents but all of it
    // later i will use INCREMENTAL but i have to implement diffs logic myself
    // atleast it will be better than full sync
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    // after initializing the lsp module it logs message in the editor
    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "rl LSP ready")
            .await;
    }

    // did the editor open new file?
    // use publish to give diagnostics
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.publish(&params.text_document.uri, &params.text_document.text)
            .await;
    }

    // did the file content change?
    // use publish to give diagnostics
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().last() {
            self.publish(&params.text_document.uri, &change.text).await;
        }
    }

    // did the editor close the file?
    // send nothing
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    // do nothing when the editor shuts down
    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

impl Backend {
    // pushes the diagnostics from pipeline to client (editors)
    async fn publish(&self, uri: &Url, source: &str) {
        let diagnostics = run_pipeline(source);

        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }
}
