//! Opaque typedef for `[T]`.

use opaque_typedef_macros::{OpaqueTypedef, OpaqueTypedefUnsized};

/// Local result type.
///
/// If the `opaque_typedef` crate has some kind of bug, this may used instead of
/// `std::result::Result` for impl of `Opaquetypedef{,Unsized}` crate, and compile will fail.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

/// Slice with at least 2 items.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, OpaqueTypedefUnsized)]
// About the necessity of `#[repr(C)]`, see <https://github.com/lo48576/opaque_typedef/issues/1>.
#[repr(C)]
// `Binary`, `Display`, `LowerHex`, `Octal`, `UpperHex` is useless for usual
// `[T]` types, but this is should not an error because it is implemeted only
// when `[T]: Binary` (or some other traits).
// These are specified here for testing purpose.
#[opaque_typedef(derive(
    AsMut(Deref, Self),
    AsRef(Deref, Self),
    Binary,
    DefaultRef,
    Deref,
    DerefMut,
    Display,
    FromInner,
    Into(Arc, Box, Inner, Rc),
    LowerHex,
    Octal,
    PartialEq(Inner),
    PartialOrd(Inner),
    UpperHex
))]
#[opaque_typedef(allow_mut_ref)]
#[opaque_typedef(validation(
    validator = "ensure_at_least_2_items",
    error_type = "TooFewItems",
    error_msg = "Failed to create `SliceAtLeast2Items`"
))]
pub struct SliceAtLeast2Items<T> {
    #[opaque_typedef(inner)]
    inner: [T],
}

impl<T> SliceAtLeast2Items<T> {
    /// Creates a new `&MyStr` from the given string slice.
    pub fn new(v: &[T]) -> &Self {
        ::opaque_typedef::OpaqueTypedefUnsized::from_inner(v)
    }

    /// Creates a new `&mut MyStr` from the given mutable string slice.
    pub fn new_mut(v: &mut [T]) -> &mut Self {
        ::opaque_typedef::OpaqueTypedefUnsized::from_inner_mut(v)
    }

    /// Returns a reference to the inner slice.
    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }
}

/// Vec with at least 2 items.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, OpaqueTypedef)]
#[opaque_typedef(derive(
    AsRef(Deref, Inner),
    Deref,
    Display,
    FromInner,
    IntoInner,
    PartialEqInner,
    PartialOrdInner
))]
#[opaque_typedef(deref(target = "[T]", deref = "Vec::<T>::as_slice"))]
#[opaque_typedef(validation(
    validator = "ensure_at_least_2_items",
    error_type = "TooFewItems",
    error_msg = "Failed to create `VecAtLeast2Items`"
))]
pub struct VecAtLeast2Items<T> {
    inner: Vec<T>,
}

impl<T> VecAtLeast2Items<T> {
    /// Creates a new `VecAtLeast2Items<T>` from the given vector.
    pub fn from_vec(v: Vec<T>) -> Self {
        ::opaque_typedef::OpaqueTypedef::from_inner(v)
    }

    /// Returns a reference to the inner slice.
    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }

    /// Returns a mutable reference to the inner slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.inner.as_mut_slice()
    }
}

// Implement `Borrow` and `ToOwned` to test `Cow<Mystr>`.
impl<T> ::std::borrow::Borrow<SliceAtLeast2Items<T>> for VecAtLeast2Items<T> {
    fn borrow(&self) -> &SliceAtLeast2Items<T> {
        SliceAtLeast2Items::<T>::new(self.as_slice())
    }
}

impl<T: Clone> ToOwned for SliceAtLeast2Items<T> {
    type Owned = VecAtLeast2Items<T>;
    fn to_owned(&self) -> Self::Owned {
        VecAtLeast2Items::<T>::from_vec(self.as_slice().to_owned())
    }
}

/// A type of an error indicating the number of the items are too few.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TooFewItems(usize);

fn ensure_at_least_2_items<U, T: AsRef<[U]>>(v: T) -> std::result::Result<T, TooFewItems> {
    let len = v.as_ref().len();
    if len >= 2 {
        Ok(v)
    } else {
        Err(TooFewItems(len))
    }
}
