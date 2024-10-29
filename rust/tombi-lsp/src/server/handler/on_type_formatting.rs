use text_position::TextPosition;
use text_size::TextSize;
use tower_lsp::lsp_types::{
    DocumentOnTypeFormattingParams, Position, Range, TextDocumentPositionParams, TextEdit,
};

use crate::toml;

pub async fn handle_on_type_formatting(
    DocumentOnTypeFormattingParams {
        text_document_position: TextDocumentPositionParams { text_document, .. },
        ..
    }: DocumentOnTypeFormattingParams,
) -> Result<Option<Vec<TextEdit>>, tower_lsp::jsonrpc::Error> {
    let source = toml::try_load(&text_document.uri)?;

    if let Ok(new_text) = formatter::format(&source) {
        if new_text != source {
            return Ok(Some(vec![TextEdit {
                range: Range::new(
                    Position::new(0, 0),
                    TextPosition::from_source(&source, TextSize::new(source.len() as u32)).into(),
                ),
                new_text,
            }]));
        }
    }
    Ok(None)
}