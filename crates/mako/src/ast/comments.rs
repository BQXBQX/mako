use swc_core::common;
use swc_core::common::comments::{Comment, Comments as CommentsTrait};
use swc_core::common::{BytePos, Span};
use swc_node_comments::SwcComments;

#[derive(Default)]
pub struct Comments(SwcComments);

impl Comments {
    pub fn get_swc_comments(&self) -> &SwcComments {
        &self.0
    }

    pub fn add_leading_comment_at(&mut self, pos: BytePos, comment: Comment) {
        self.0.add_leading(pos, comment);
    }

    /**
     * Check for `/*#__UNUSED__*/`
     */
    #[allow(dead_code)]
    pub fn has_unused(&self, span: Span) -> bool {
        self.has_flag(span, "UNUSED")
    }

    /**
     * Check for `/*#__UNUSED_MODULE__*/`
     */
    #[allow(dead_code)]
    pub fn has_unused_module(&self, span: Span) -> bool {
        self.has_flag(span, "UNUSED_MODULE")
    }

    /**
     * Check for `/*#__PURE__*/`
     */
    #[allow(dead_code)]
    pub fn has_pure(&self, span: Span) -> bool {
        self.has_flag(span, "PURE")
    }

    /**
     * Check for `/*#__NO_SIDE_EFFECTS__*/`
     */
    #[allow(dead_code)]
    fn has_no_side_effects(&self, span: Span) -> bool {
        self.has_flag(span, "NO_SIDE_EFFECTS")
    }

    #[allow(dead_code)]
    fn has_flag(&self, span: Span, text: &'static str) -> bool {
        self.find_comment(span, |c| {
            if c.kind == common::comments::CommentKind::Block {
                //
                if c.text.len() == (text.len() + 5)
                    && (c.text.starts_with("#__") || c.text.starts_with("@__"))
                    && c.text.ends_with("__")
                    && text == &c.text[3..c.text.len() - 2]
                {
                    return true;
                }
            }

            false
        })
    }

    #[allow(dead_code)]
    fn find_comment<F>(&self, span: Span, mut op: F) -> bool
    where
        F: FnMut(&common::comments::Comment) -> bool,
    {
        let mut found = false;
        let cs: Option<_> = common::comments::Comments::get_leading(&self.0, span.lo);
        if let Some(cs) = cs {
            for c in &cs {
                found |= op(c);
                if found {
                    break;
                }
            }
        }

        found
    }
}
