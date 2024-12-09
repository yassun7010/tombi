#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean {
    value: bool,
    range: text::Range,
    symbol_range: text::Range,
}

impl Boolean {
    #[inline]
    pub fn value(&self) -> bool {
        self.value
    }

    #[inline]
    pub fn range(&self) -> text::Range {
        self.range
    }

    #[inline]
    pub fn symbol_range(&self) -> text::Range {
        self.symbol_range
    }
}

impl TryFrom<ast::Boolean> for Boolean {
    type Error = Vec<crate::Error>;

    fn try_from(node: ast::Boolean) -> Result<Self, Self::Error> {
        let token = node.token().unwrap();
        Ok(Self {
            value: match token.text() {
                "true" => true,
                "false" => false,
                _ => unreachable!(),
            },
            range: token.range(),
            symbol_range: token.range(),
        })
    }
}
