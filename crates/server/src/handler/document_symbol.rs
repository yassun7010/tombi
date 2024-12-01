use crate::backend::Backend;
use ast::AstNode;
use config::TomlVersion;
use document::{Document, Value};
use tower_lsp::lsp_types::{
    DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse, SymbolKind,
};

#[tracing::instrument(level = "debug", skip_all)]
pub async fn handle_document_symbol(
    backend: &Backend,
    DocumentSymbolParams { text_document, .. }: DocumentSymbolParams,
) -> Result<Option<DocumentSymbolResponse>, tower_lsp::jsonrpc::Error> {
    tracing::info!("handle_document_symbol");

    let Some(document) = backend.documents.get(&text_document.uri) else {
        return Ok(None);
    };

    let p = parser::parse(&document.source, TomlVersion::default());
    if !p.errors().is_empty() {
        return Ok(None);
    }

    let root = ast::Root::cast(p.into_syntax_node()).unwrap();
    let Ok(document) = document::Document::try_from(root) else {
        return Ok(None);
    };

    let symbols = create_symbols(&document);

    tracing::debug!("DocumentSymbols: {symbols:#?}");

    Ok(Some(DocumentSymbolResponse::Nested(symbols)))
}

fn create_symbols(document: &Document) -> Vec<DocumentSymbol> {
    let mut symbols: Vec<DocumentSymbol> = vec![];

    for (key, value) in document.key_values() {
        symbols_for_value(key.to_string(), value, None, &mut symbols);
    }

    symbols
}

#[allow(deprecated)]
fn symbols_for_value(
    name: String,
    value: &Value,
    parent_key_range: Option<text::Range>,
    symbols: &mut Vec<DocumentSymbol>,
) {
    let value_range = value.range();
    let range = if let Some(parent_key_range) = parent_key_range {
        parent_key_range + value_range
    } else {
        value_range
    };

    let selection_range = range;

    match value {
        Value::Boolean { .. } => {
            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::BOOLEAN,
                range: range.into(),
                selection_range: selection_range.into(),
                children: None,
                detail: None,
                deprecated: None,
                tags: None,
            });
        }
        Value::Integer { .. } => {
            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::NUMBER,
                range: range.into(),
                selection_range: selection_range.into(),
                children: None,
                detail: None,
                deprecated: None,
                tags: None,
            });
        }
        Value::Float { .. } => {
            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::NUMBER,
                range: range.into(),
                selection_range: selection_range.into(),
                children: None,
                detail: None,
                deprecated: None,
                tags: None,
            });
        }
        Value::String { .. } => {
            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::STRING,
                range: range.into(),
                selection_range: selection_range.into(),
                children: None,
                detail: None,
                deprecated: None,
                tags: None,
            });
        }
        Value::OffsetDateTime { .. }
        | Value::LocalDateTime { .. }
        | Value::LocalDate { .. }
        | Value::LocalTime { .. } => {
            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::STRING,
                range: range.into(),
                selection_range: selection_range.into(),
                children: None,
                detail: None,
                deprecated: None,
                tags: None,
            });
        }
        Value::Array(array) => {
            let mut children = vec![];
            for (index, value) in array.values().iter().enumerate() {
                symbols_for_value(index.to_string(), value, Some(range), &mut children);
            }

            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::ARRAY,
                range: range.into(),
                selection_range: selection_range.into(),
                children: Some(children),
                detail: None,
                deprecated: None,
                tags: None,
            });
        }
        Value::Table(table) => {
            let mut children = vec![];
            for (key, value) in table.key_values() {
                symbols_for_value(key.to_string(), value, Some(range), &mut children);
            }

            symbols.push(DocumentSymbol {
                name,
                kind: SymbolKind::OBJECT,
                range: range.into(),
                selection_range: selection_range.into(),
                children: Some(children),
                detail: None,
                deprecated: None,
                tags: None,
            });
        }
    }
}
