use tower_lsp::jsonrpc::Result as RpcResult;
use tower_lsp::{
	lsp_types::{
		DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
		InitializeParams, InitializeResult, InitializedParams, MessageType, ServerCapabilities,
	},
	Client, LanguageServer, LspService, Server,
};

#[derive(Debug)]
struct Backend {
	client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
	async fn initialize(&self, _: InitializeParams) -> RpcResult<InitializeResult> {
		Ok(InitializeResult {
			server_info: None,
			offset_encoding: None,
			capabilities: ServerCapabilities {
				..ServerCapabilities::default()
			},
		})
	}

	async fn initialized(&self, _: InitializedParams) {
		self.client.log_message(MessageType::INFO, "initialized").await;
	}

	async fn shutdown(&self) -> RpcResult<()> {
		Ok(())
	}

	async fn did_open(&self, _params: DidOpenTextDocumentParams) {
		self.client.log_message(MessageType::INFO, "did_open").await;
	}

	async fn did_change(&self, _params: DidChangeTextDocumentParams) {
		self.client.log_message(MessageType::INFO, "did_change").await;
	}

	async fn did_save(&self, _params: DidSaveTextDocumentParams) {
		self.client.log_message(MessageType::INFO, "did_save").await;
	}

	async fn did_close(&self, _params: DidCloseTextDocumentParams) {
		self.client.log_message(MessageType::INFO, "did_close").await;
	}
}

#[tokio::main]
async fn main() {
	let stdin = tokio::io::stdin();
	let stdout = tokio::io::stdout();

	let (service, sock) = LspService::build(|client| Backend { client }).finish();

	serde_json::json!({"test": 20});
	Server::new(stdin, stdout, sock).serve(service).await;
}
