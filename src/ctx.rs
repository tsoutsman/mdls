use crate::{index::LineIndex, Error, Result};
use crossbeam_channel::{Receiver, Sender};
use hashbrown::HashMap;
use lsp_server::{Connection, Incoming, Message, Outgoing, ReqQueue, Request, Response};
use lsp_types::Url;

pub struct Context {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
    incoming: Incoming<()>,
    outgoing: Outgoing<fn(&mut Context, Response)>,
    files: HashMap<Url, (LineIndex, String)>,
    _io_threads: lsp_server::IoThreads,
}

impl std::fmt::Debug for Context {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Context {
    #[must_use]
    pub fn init_from_stdio() -> Self {
        let (connection, io_threads) = Connection::stdio();

        let server_capabilities = serde_json::to_value(&lsp_types::ServerCapabilities {
            document_formatting_provider: Some(lsp_types::OneOf::Left(true)),
            text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
                lsp_types::TextDocumentSyncKind::FULL,
            )),
            ..Default::default()
        })
        .unwrap();
        let _initialization_params = connection.initialize(server_capabilities).unwrap();

        let ReqQueue { incoming, outgoing } = ReqQueue::default();
        Self {
            sender: connection.sender,
            receiver: connection.receiver,
            incoming,
            outgoing,
            files: HashMap::new(),
            _io_threads: io_threads,
        }
    }

    #[must_use]
    pub fn next_event(&self) -> Option<Message> {
        self.receiver.recv().ok()
    }

    pub(crate) fn get_contents(&self, uri: &Url) -> Result<(&LineIndex, &str)> {
        self.files
            .get(uri)
            .map(|(index, uri)| (index, uri.as_ref()))
            .ok_or(Error::UnknownDocument)
    }

    pub(crate) fn get_mut_contents(&mut self, uri: &Url) -> Result<&mut (LineIndex, String)> {
        self.files
            .get_mut(uri)
            // .map(|(index, uri)| (index, uri.as_ref()))
            .ok_or(Error::UnknownDocument)
    }

    pub(crate) fn insert_file(&mut self, uri: Url, text: String) -> Result<()> {
        let line_index = LineIndex::new(&text);
        match self.files.insert(uri, (line_index, text)) {
            None => Ok(()),
            Some(_) => Err(Error::DocumentAlreadyExists),
        }
    }

    /// Sends a request to the client.
    pub fn send_request<R>(&mut self, params: R::Params, handler: fn(&mut Context, Response))
    where
        R: lsp_types::request::Request,
    {
        let request = self
            .outgoing
            .register(R::METHOD.to_owned(), params, handler);
        self.send(request.into());
    }

    /// Handles a client response.
    pub fn complete_request(&mut self, response: Response) {
        let handler = self
            .outgoing
            .complete(response.id.clone())
            .expect("received response for unknown request");
        handler(self, response);
    }

    /// Register an incoming request.
    pub fn register_request(&mut self, request: Request) {
        self.incoming.register(request.id, ());
    }

    /// Respond to an incoming request.
    pub fn respond(&mut self, response: Response) {
        self.incoming
            .complete(response.id.clone())
            .expect("responding to unknown request");
        self.send(response.into());
    }

    fn send(&mut self, message: Message) {
        self.sender.send(message).unwrap();
    }
}
