// The final arm is the single backend inventory. Its order determines benchmark
// schedules; the other arms expand that same inventory into types or a match.
macro_rules! match_backends {
    (@declare; $( $variant:ident($tokenizer:ty) => $label:literal, )*) => {
        /// Identifies an implementation before loading it or warming its caches.
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum TokenizerBackend {
            $( $variant, )*
        }

        impl TokenizerBackend {
            /// Every declared backend in the fixed benchmark scheduling order.
            const ALL: &'static [Self] = &[$( Self::$variant, )*];

            /// Returns the stable implementation label used by result readers.
            const fn label(self) -> &'static str {
                match self {
                    $( Self::$variant => $label, )*
                }
            }
        }

        enum Engine {
            $( $variant($tokenizer), )*
        }

        impl Engine {
            /// Derives identity from the loaded variant without touching its tokenizer.
            fn backend(&self) -> TokenizerBackend {
                match_backends!(self, |backend, _tokenizer| backend)
            }
        }
    };
    (@$engine:expr, |$backend:ident, $tokenizer:ident| $body:expr;
        $( $variant:ident($native:ty) => $label:literal, )*
    ) => {
        match $engine {
            $( Engine::$variant($tokenizer) => {
                let $backend = TokenizerBackend::$variant;
                $body
            }, )*
        }
    };
    ($($operation:tt)*) => {
        match_backends! {
            @$($operation)*;
            Snaptokens(snaptokens::Tokenizer) => "snaptokens",
            Fastokens(fastokens::Tokenizer) => "fastokens",
            HuggingFace(tokenizers::Tokenizer) => "huggingface",
            Gigatoken(Gigatoken) => "gigatoken",
            Iree(iree_tokenizer::Tokenizer) => "iree",
            QuickTok(QuickTok) => "quicktok-qwen3-c-abi",
            Kitoken(kitoken::Kitoken) => "kitoken",
            Tokie(tokie::Tokenizer) => "tokie",
            Splintr(splintr::AnyTokenizer) => "splintr",
        }
    };
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, rc::Rc};

    // Stand-ins let the production macro run without loading third-party native
    // libraries. Each payload records destruction so identity lookup cannot hide a move.
    struct Payload(Rc<Cell<usize>>);

    impl Drop for Payload {
        /// Records when native ownership ends during the borrowing check.
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    mod snaptokens {
        pub(super) type Tokenizer = super::Payload;
    }
    mod fastokens {
        pub(super) type Tokenizer = super::Payload;
    }
    mod tokenizers {
        pub(super) type Tokenizer = super::Payload;
    }
    mod iree_tokenizer {
        pub(super) type Tokenizer = super::Payload;
    }
    mod kitoken {
        pub(super) type Kitoken = super::Payload;
    }
    mod tokie {
        pub(super) type Tokenizer = super::Payload;
    }
    mod splintr {
        pub(super) type AnyTokenizer = super::Payload;
    }
    type Gigatoken = Payload;
    type QuickTok = Payload;

    match_backends!(declare);

    /// Pins the serialized labels and scheduling order independently of the declaration.
    #[test]
    fn backend_inventory_preserves_result_contract() {
        let labels: Vec<_> = TokenizerBackend::ALL
            .iter()
            .map(|backend| backend.label())
            .collect();
        assert_eq!(
            labels,
            [
                "snaptokens",
                "fastokens",
                "huggingface",
                "gigatoken",
                "iree",
                "quicktok-qwen3-c-abi",
                "kitoken",
                "tokie",
                "splintr",
            ]
        );
    }

    /// Checks every mapping and verifies that repeated identity reads retain native ownership.
    #[test]
    fn loaded_engines_report_their_own_backend() {
        let drops = Rc::new(Cell::new(0));
        let payload = || Payload(Rc::clone(&drops));
        let engines = [
            Engine::Snaptokens(payload()),
            Engine::Fastokens(payload()),
            Engine::HuggingFace(payload()),
            Engine::Gigatoken(payload()),
            Engine::Iree(payload()),
            Engine::QuickTok(payload()),
            Engine::Kitoken(payload()),
            Engine::Tokie(payload()),
            Engine::Splintr(payload()),
        ];
        assert_eq!(engines.len(), TokenizerBackend::ALL.len());
        for (engine, expected) in engines.iter().zip(TokenizerBackend::ALL) {
            assert!(engine.backend() == *expected);
            assert!(engine.backend() == *expected);
            let mut evaluations = 0;
            let label = match_backends!(
                {
                    evaluations += 1;
                    engine
                },
                |backend, tokenizer| {
                    assert!(Rc::ptr_eq(&tokenizer.0, &drops));
                    backend.label()
                }
            );
            assert_eq!(evaluations, 1);
            assert_eq!(label, expected.label());
        }
        assert_eq!(drops.get(), 0);
        drop(engines);
        assert_eq!(drops.get(), TokenizerBackend::ALL.len());
    }
}
