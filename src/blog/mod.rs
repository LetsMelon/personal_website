use html_site_generator::html::body::Body;
use html_site_generator::html::document::Document;

use crate::utils::{footer, header, navbar};

mod inner_body;

pub fn build() -> Document {
    let head = header::build("blogs");

    let mut body = Body::new();

    body.add_element(navbar::build());
    body.add_element(inner_body::build());
    body.add_element(footer::build());

    Document::new(head, body)
}
