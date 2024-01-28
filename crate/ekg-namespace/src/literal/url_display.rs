use {
    crate::literal::this::Literal,
    std::fmt::{Display, Formatter},
};

pub struct LiteralUrlDisplay<'a> {
    pub(crate) literal: &'a Literal,
}

impl<'a> Display for LiteralUrlDisplay<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.literal.data_type.is_string() {
            write!(
                f,
                "{}",
                serde_urlencoded::to_string(self.literal.as_str().unwrap_or(""))
                    .map_err(|_| std::fmt::Error)?
            )
        } else if self.literal.data_type.is_boolean() {
            write!(
                f,
                "{:}",
                self.literal.as_boolean().unwrap_or(false)
            )
        } else {
            write!(f, "{:}", self.literal.to_string().as_str())
        }
    }
}
