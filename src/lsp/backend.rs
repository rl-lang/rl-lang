use std::collections::HashMap;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::lsp::hover::run_hover;
use crate::lsp::pipeline::run_pipeline;

pub struct Backend {
    pub client: Client,
    pub docs: RwLock<HashMap<Url, String>>,
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
                hover_provider: Some(HoverProviderCapability::Simple(true)),
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
    // cache the text for hover then give diagnostics
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.store(&params.text_document.uri, &params.text_document.text)
            .await;
        self.publish(&params.text_document.uri, &params.text_document.text)
            .await;
    }

    // did the file content change?
    // cache the text for hover then give diagnostics
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().last() {
            self.store(&params.text_document.uri, &change.text).await;
            self.publish(&params.text_document.uri, &change.text).await;
        }
    }

    // did the editor close the file?
    // forget the cached text and send no diagnostics
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.docs.write().await.remove(&params.text_document.uri);
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    // editor asked what is under the cursor
    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let docs = self.docs.read().await;
        let Some(source) = docs.get(uri) else {
            return Ok(None);
        };

        Ok(run_hover(source, position))
    }

    // do nothing when the editor shuts down
    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

impl Backend {
    // caches the latest full text for a document so hover() has
    // something to read later
    async fn store(&self, uri: &Url, source: &str) {
        self.docs
            .write()
            .await
            .insert(uri.clone(), source.to_string());
    }

    // pushes the diagnostics from pipeline to client (editors)
    async fn publish(&self, uri: &Url, source: &str) {
        let diagnostics = run_pipeline(source);

        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }
}
