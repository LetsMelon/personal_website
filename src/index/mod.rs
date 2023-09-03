use html_site_generator::html::body::Body;
use html_site_generator::html::document::Document;

use crate::utils::{footer, header, navbar};

mod button;
mod inner_body;
mod project;

pub fn build() -> Document {
    let head = header::build();

    let mut body = Body::new();

    body.add_element(navbar::build());
    // TODO `<span>AGE</span>`
    body.add_element(inner_body::build());
    body.add_element(footer::build());

    Document::new(head, body)
}
