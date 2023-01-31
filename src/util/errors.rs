/// Trivial macro implementing From<`from`> for `to`, using the enum variant `variant`
macro_rules! error_from {
    ($to: ty, $from: ty, $variant: expr) => {
        impl From<$from> for $to {
            fn from(error: $from) -> Self {
                $variant(error)
            }
        }
    };
}
pub(crate) use error_from;
