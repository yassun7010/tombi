use super::Format;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeginDanglingComment(pub ast::Comment);

impl Format for BeginDanglingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}\n", f.ident(), self.0)
    }
}

impl Format for Vec<BeginDanglingComment> {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        for comment in self {
            comment.fmt(f)?;
        }
        write!(f, "\n")?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndDanglingComment(pub ast::Comment);

impl Format for EndDanglingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "\n{}{}", f.ident(), self.0)
    }
}

impl Format for Vec<EndDanglingComment> {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        if self.is_empty() {
            return Ok(());
        }

        write!(f, "\n")?;

        for comment in self {
            comment.fmt(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeadingComment(pub ast::Comment);

impl Format for LeadingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}\n", f.ident(), self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TailingComment(pub ast::Comment);

impl Format for TailingComment {
    #[inline]
    fn fmt(&self, f: &mut crate::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}", f.defs().tailing_comment_space(), self.0)
    }
}
