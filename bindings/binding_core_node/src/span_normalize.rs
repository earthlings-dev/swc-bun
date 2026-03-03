use swc_core::{
    common::{BytePos, Span},
    ecma::visit::VisitMut,
};

/// Normalizes all `Span` positions in an AST to be 0-based byte offsets
/// relative to the source file.
///
/// `SourceMap` assigns monotonically increasing `BytePos` ranges starting at 1.
/// The parse API serializes the AST to JSON, so consumers expect spans to be
/// 0-based offsets into the source text, not global `SourceMap` positions.
pub struct SpanNormalizer {
    offset: u32,
}

impl SpanNormalizer {
    pub fn new(file_start_pos: BytePos) -> Self {
        SpanNormalizer {
            offset: file_start_pos.0,
        }
    }
}

impl VisitMut for SpanNormalizer {
    fn visit_mut_span(&mut self, span: &mut Span) {
        if span.is_dummy() {
            return;
        }

        span.lo = BytePos(span.lo.0 - self.offset);
        span.hi = BytePos(span.hi.0 - self.offset);
    }
}

/// Reverses span normalization: shifts 0-based byte offsets back to
/// `SourceMap`-relative positions so that `lookup_char_pos` can find the file.
///
/// Used when a deserialized AST (whose spans were normalized by
/// [`SpanNormalizer`]) needs to be fed back into `Compiler::process_js`.
pub struct SpanDenormalizer {
    pub offset: u32,
}

impl VisitMut for SpanDenormalizer {
    fn visit_mut_span(&mut self, span: &mut Span) {
        if span.is_dummy() {
            return;
        }

        span.lo = BytePos(span.lo.0 + self.offset);
        span.hi = BytePos(span.hi.0 + self.offset);
    }
}

#[cfg(test)]
mod tests {
    use swc_core::{
        common::{Span, DUMMY_SP},
        ecma::visit::VisitMutWith,
    };

    use super::*;

    #[test]
    fn normalizes_regular_span() {
        let mut span = Span::new(BytePos(1), BytePos(13));
        span.visit_mut_with(&mut SpanNormalizer::new(BytePos(1)));
        assert_eq!(span.lo.0, 0);
        assert_eq!(span.hi.0, 12);
    }

    #[test]
    fn preserves_dummy_span() {
        let mut span = DUMMY_SP;
        span.visit_mut_with(&mut SpanNormalizer::new(BytePos(1)));
        assert_eq!(span.lo.0, 0);
        assert_eq!(span.hi.0, 0);
    }

    #[test]
    fn normalizes_with_accumulated_offset() {
        let mut span = Span::new(BytePos(1000), BytePos(1050));
        span.visit_mut_with(&mut SpanNormalizer::new(BytePos(1000)));
        assert_eq!(span.lo.0, 0);
        assert_eq!(span.hi.0, 50);
    }

    #[test]
    fn denormalizes_regular_span() {
        let mut span = Span::new(BytePos(0), BytePos(12));
        span.visit_mut_with(&mut SpanDenormalizer { offset: 1 });
        assert_eq!(span.lo.0, 1);
        assert_eq!(span.hi.0, 13);
    }

    #[test]
    fn denormalize_preserves_dummy_span() {
        let mut span = DUMMY_SP;
        span.visit_mut_with(&mut SpanDenormalizer { offset: 1 });
        assert_eq!(span.lo.0, 0);
        assert_eq!(span.hi.0, 0);
    }

    #[test]
    fn roundtrip_normalize_denormalize() {
        let mut span = Span::new(BytePos(500), BytePos(600));
        span.visit_mut_with(&mut SpanNormalizer::new(BytePos(500)));
        assert_eq!(span.lo.0, 0);
        assert_eq!(span.hi.0, 100);

        span.visit_mut_with(&mut SpanDenormalizer { offset: 1 });
        assert_eq!(span.lo.0, 1);
        assert_eq!(span.hi.0, 101);
    }
}
