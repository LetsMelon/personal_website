use html_site_generator::html::body::Body;
use html_site_generator::html::document::Document;

mod footer;
mod header;
mod inner_body;
mod navbar;

pub fn build() -> Document {
    let head = header::build();

    let mut body = Body::new();

    body.add_element(navbar::build());
    // TODO `<span>AGE</span>`
    body.add_element(inner_body::build());
    body.add_element(footer::build());

    Document::new(head, body)
}
