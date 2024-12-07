use crate::{ArrayOfTable, AstNode, Keys, Table};

#[derive(Debug, Clone)]
pub enum TableOrArrayOfTable {
    Table(Table),
    ArrayOfTable(ArrayOfTable),
}

impl TableOrArrayOfTable {
    pub fn header(&self) -> Option<Keys> {
        match self {
            TableOrArrayOfTable::Table(table) => table.header(),
            TableOrArrayOfTable::ArrayOfTable(array_of_table) => array_of_table.header(),
        }
    }

    pub fn range(&self) -> text::Range {
        match self {
            TableOrArrayOfTable::Table(table) => table.range(),
            TableOrArrayOfTable::ArrayOfTable(array_of_table) => array_of_table.range(),
        }
    }
}

impl AstNode for TableOrArrayOfTable {
    #[inline]
    fn can_cast(kind: syntax::SyntaxKind) -> bool {
        Table::can_cast(kind) || ArrayOfTable::can_cast(kind)
    }

    #[inline]
    fn cast(syntax: syntax::SyntaxNode) -> Option<Self> {
        if Table::can_cast(syntax.kind()) {
            Table::cast(syntax).map(TableOrArrayOfTable::Table)
        } else if ArrayOfTable::can_cast(syntax.kind()) {
            ArrayOfTable::cast(syntax).map(TableOrArrayOfTable::ArrayOfTable)
        } else {
            None
        }
    }

    #[inline]
    fn syntax(&self) -> &syntax::SyntaxNode {
        match self {
            TableOrArrayOfTable::Table(table) => table.syntax(),
            TableOrArrayOfTable::ArrayOfTable(array_of_table) => array_of_table.syntax(),
        }
    }
}
