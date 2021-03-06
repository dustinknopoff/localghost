//! The browser's Document Object Model.
//!
//! # Examples
//!
//! ```no_run
//! use localghost::prelude::*;
//! use localghost::dom::{body, Element};
//!
//! #[localghost::main]
//! async fn main() {
//!     let el = Element::with_text("p", "Hello world");
//!     body().append(el);
//! }
//! ```

// re-exports, temporary only
pub use element::Element;
pub use query_selector::query_selector;
pub use text::Text;
pub use window::Window;

mod element;
mod query_selector;
mod text;
mod window;

use crate::prelude::*;

/// Get the document's body.
pub fn body() -> Element {
    let el = crate::utils::document()
        .query_selector("body")
        .expect_throw("Could not find `window.body`")
        .expect_throw("Could not find `window.body`");
    unsafe { Element::from_raw("body", el) }
}

/// Wait for the DOM to be loaded.
///
/// # Examples
///
/// ```no_run
/// use wasm_bindgen::prelude::*;
/// use localghost::dom;
///
/// #[localghost::main]
/// async fn main() {
///     println!("waiting on document to load");
///     dom::ready().await;
///     println!("document loaded!");
/// }
/// ```
pub async fn ready() {
    let doc = crate::utils::document();
    match doc.ready_state().as_str() {
        "complete" | "interactive" => return,
        _ => doc.once("DOMContentLoaded").await,
    };
}
