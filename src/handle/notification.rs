use crate::{index::LineIndex, Context, Result};
use lsp_types::notification::{self, Notification};

pub fn did_open_text_document(
    ctx: &mut Context,
    params: <notification::DidOpenTextDocument as Notification>::Params,
) -> Result<()> {
    ctx.insert_file(params.text_document.uri, params.text_document.text)
}

pub fn did_change_text_document(
    ctx: &mut Context,
    params: <notification::DidChangeTextDocument as Notification>::Params,
) -> Result<()> {
    let uri = params.text_document.uri;
    assert_eq!(params.content_changes.len(), 1);
    let text = params.content_changes.into_iter().next().unwrap().text;
    let contents = ctx.get_mut_contents(&uri)?;
    *contents = (LineIndex::new(&text), text);
    Ok(())
}
