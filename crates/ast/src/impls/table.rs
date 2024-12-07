use crate::{support, AstChildren, AstNode, TableOrArrayOfTable};
use syntax::T;

impl crate::Table {
    pub fn header_leading_comments(&self) -> impl Iterator<Item = crate::Comment> {
        support::leading_comments(self.syntax().children_with_tokens())
    }

    pub fn header_tailing_comment(&self) -> Option<crate::Comment> {
        support::tailing_comment(self.syntax().children_with_tokens(), T!(']'))
    }

    /// Returns an iterator over the subtables of this table.
    ///
    /// ```toml
    /// [foo]  # <- This is a self table
    /// [foo.bar]  # <- This is a subtable
    /// key = "value"
    ///
    /// [[foo.bar.baz]]  # <- This is also a subtable
    /// key = true
    /// ```
    pub fn subtables<'a>(&'a self) -> impl Iterator<Item = TableOrArrayOfTable> + 'a {
        support::next_siblings_nodes(self).take_while(|t: &TableOrArrayOfTable| {
            t.header()
                .unwrap()
                .keys()
                .starts_with(&self.header().unwrap().keys())
        })
    }

    fn parent_tables<'a>(&'a self) -> impl Iterator<Item = TableOrArrayOfTable> + 'a {
        support::prev_siblings_nodes(self).take_while(|t: &TableOrArrayOfTable| {
            self.header()
                .unwrap()
                .keys()
                .starts_with(&t.header().unwrap().keys())
        })
    }

    pub fn array_of_tables_keys<'a>(
        &'a self,
    ) -> impl Iterator<Item = AstChildren<crate::Key>> + 'a {
        self.parent_tables()
            .filter_map(|parent_table| match parent_table {
                crate::TableOrArrayOfTable::ArrayOfTables(array_of_tables) => {
                    Some(array_of_tables.header().unwrap().keys())
                }
                _ => None,
            })
    }
}
