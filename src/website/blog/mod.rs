use html_site_generator::html::body::Body;
use html_site_generator::html::document::Document;

use crate::website::utils::{footer, header, navbar};
use crate::website::widget::custom_document::CustomDocument;

mod inner_body;

pub fn build() -> CustomDocument {
    let head = header::build("blogs");

    let mut body = Body::new();

    body.add_element(navbar::build());
    body.add_element(inner_body::build());
    body.add_element(footer::build());

    CustomDocument::new(Document::new(head, body))
}
