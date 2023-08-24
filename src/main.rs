use std::io::Write;

use html_site_generator::html::body::Body;
use html_site_generator::html::document::Document;
use html_site_generator::html::IntoHtmlNode;

mod footer;
mod header;
mod inner_body;
mod navbar;

fn main() {
    let head = header::build();

    let mut body = Body::new();

    body.add_element(navbar::build());
    // TODO `<span>AGE</span>`
    body.add_element(inner_body::build());
    body.add_element(footer::build());

    let document = Document::new(head, body);

    let mut output = Box::new(std::io::stdout()) as Box<dyn Write>;
    document.transform_into_html_node(&mut output).unwrap();
}
