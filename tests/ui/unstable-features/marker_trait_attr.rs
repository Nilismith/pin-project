// compile-fail

// NB: If you change this test, change 'marker_trait_attr-feature-gate.rs' at the same time.

// marker_trait_attr
// Tracking issue: https://github.com/rust-lang/rust/issues/29864
#![feature(marker_trait_attr)]

// See https://github.com/taiki-e/pin-project/issues/105#issuecomment-535355974

use pin_project::pin_project;
use std::marker::PhantomPinned;

#[pin_project] //~ ERROR E0119
struct Foo<T> {
    #[pin]
    x: T,
}

// unsound Unpin impl
impl<T> Unpin for Foo<T> {}

fn is_unpin<T: Unpin>() {}

fn foo_is_unpin() {
    is_unpin::<Foo<PhantomPinned>>()
}

fn main() {}