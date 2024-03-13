use html_site_generator::html::body::Body;
use html_site_generator::html::document::Document;

use crate::utils::{footer, header, navbar};
use crate::widget::custom_document::CustomDocument;

mod employment;
mod inner_body;
mod project;

pub fn build() -> CustomDocument {
    let head = header::build("index");

    let mut body = Body::new();

    body.add_element(navbar::build());
    // TODO `<span>AGE</span>`
    body.add_element(inner_body::build());
    body.add_element(footer::build());

    CustomDocument::new(Document::new(head, body))
}
