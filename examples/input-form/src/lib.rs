use dodrio::{bumpalo, Node, Render, RenderContext};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Say hello to someone.
struct SayHelloTo {
    /// Who to say hello to.
    who: String,
}

impl SayHelloTo {
    /// Construct a new `SayHelloTo` component.
    fn new<S: Into<String>>(who: S) -> SayHelloTo {
        let who = who.into();
        SayHelloTo { who }
    }

    /// Update who to say hello to.
    fn set_who(&mut self, who: String) {
        self.who = who;
    }
}

// The `Render` implementation has a text `<input>` and a `<div>` that shows a
// greeting to the `<input>`'s value.
impl<'a> Render<'a> for SayHelloTo {
    fn render(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        use dodrio::builder::*;

        div(&cx)
            .children([
                input(&cx)
                    .attr("type", "text")
                    .attr(
                        "value",
                        bumpalo::format!(in cx.bump, "{}", self.who).into_bump_str(),
                    )
                    .on("input", |root, vdom, event| {
                        // If the event's target is our input...
                        let input = match event
                            .target()
                            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                        {
                            None => return,
                            Some(input) => input,
                        };

                        // ...then get its value and update who we are greeting.
                        let value = input.value();
                        let hello = root.unwrap_mut::<SayHelloTo>();
                        hello.set_who(value);

                        // Finally, re-render the component on the next animation frame.
                        vdom.schedule_render();
                    })
                    .finish(),
                text(bumpalo::format!(in cx.bump, "Hello, {}!", self.who).into_bump_str()),
            ])
            .finish()
    }
}

use localghost::prelude::*;
use localghost::dom::{Element, ElementKind, Text};

#[localghost::main]
async fn main() {
    let who = "World";

    let body = localghost::document().body();
    let input = Element::with_text(ElementKind::Input, who);
    input.set_attribute("type", "text");
    input.set_attribute("value", who);

    body.append_child(p);
}
