use syntax::SyntaxKind;

#[allow(non_camel_case_types)]
type bits = u64;

#[derive(Debug, Default)]
pub struct Lexed {
    pub kinds: Vec<SyntaxKind>,
    pub joints: Vec<bits>,
    pub errors: Vec<crate::Error>,
}

impl Lexed {
    #[inline]
    pub fn push_kind(&mut self, kind: SyntaxKind) {
        let idx = self.len();
        if idx % (bits::BITS as usize) == 0 {
            self.joints.push(0);
        }
        self.kinds.push(kind);
    }

    #[inline]
    pub fn push_error(&mut self, error: crate::Error) {
        self.errors.push(error);
    }

    fn bit_index(&self, n: usize) -> (usize, usize) {
        let idx = n / (bits::BITS as usize);
        let b_idx = n % (bits::BITS as usize);
        (idx, b_idx)
    }

    fn len(&self) -> usize {
        self.kinds.len()
    }

    /// Sets jointness for the last token we've pushed.
    ///
    /// This is a separate API rather than an argument to the `push` to make it
    /// convenient both for textual and mbe tokens. With text, you know whether
    /// the *previous* token was joint, with mbe, you know whether the *current*
    /// one is joint. This API allows for styles of usage:
    #[inline]
    pub fn set_joint(&mut self) {
        let n = self.len() - 1;
        let (idx, b_idx) = self.bit_index(n);
        self.joints[idx] |= 1 << b_idx;
    }
}
