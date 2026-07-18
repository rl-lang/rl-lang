//! [`LanguageServer`] implementation - the JSON-RPC entry point for all LSP events.
//!
//! [`Backend`] holds a [`RwLock`]-guarded map of open document URIs to their
//! latest source text. Every file event updates this cache so [`hover`] always
//! has the most recent content to work with.

use crate::goto_definition::run_goto_definition;
use crate::hover::run_hover;
use crate::pipeline::run_pipeline;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

/// The LSP backend, wiring tower-lsp events to the rl pipeline.
pub struct Backend {
    /// The tower-lsp client handle used to send messages and diagnostics back to the editor.
    pub client: Client,
    /// Cache of open document source text, keyed by URI.
    ///
    /// Written on `didOpen`/`didChange`, read on `hover`, cleared on `didClose`.
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
                definition_provider: Some(OneOf::Left(true)),
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

        Ok(run_hover(source, position, uri))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let docs = self.docs.read().await;
        let Some(source) = docs.get(uri) else {
            return Ok(None);
        };

        Ok(run_goto_definition(source, position, uri))
    }

    // do nothing when the editor shuts down
    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

impl Backend {
    /// Inserts or updates the cached source text for `uri`.
    async fn store(&self, uri: &Url, source: &str) {
        self.docs
            .write()
            .await
            .insert(uri.clone(), source.to_string());
    }

    /// Runs the rl pipeline on `source` and publishes the resulting diagnostics to the editor.
    async fn publish(&self, uri: &Url, source: &str) {
        let diagnostics = run_pipeline(source, uri);

        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }
}
