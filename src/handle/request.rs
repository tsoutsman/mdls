use crate::{proto, Context, Result};
use lsp_types::request::{self, Request};

pub fn format(
    ctx: &mut Context,
    request: <request::Formatting as Request>::Params,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    let uri = request.text_document.uri;
    let (line_index, contents) = ctx.get_contents(&uri)?;
    let edit = crate::fmt(contents);
    let vec = proto::text_edit_vec(line_index, edit);
    if vec.is_empty() {
        Ok(None)
    } else {
        Ok(Some(vec))
    }
}
