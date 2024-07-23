use html_site_generator::html::body::Body;
use html_site_generator::html::div::Div;
use html_site_generator::html::document::Document;
use html_site_generator::html::paragraph::Paragraph;
use html_site_generator::html::text::{TextElement, TextElementStyling};

use crate::website::utils::{footer, header, navbar};
use crate::website::widget::custom_document::CustomDocument;

pub fn build() -> CustomDocument {
    let head = header::build("blogs");

    let mut body = Body::new();

    body.add_element(navbar::build());
    body.add_element({
        let mut d = Div::new();

        d.add_element({
            let mut p = Paragraph::new();

            p.add_element(TextElement::new_with_styling(
                "Ideas around WASM",
                TextElementStyling::Bold,
            ));

            p.add_element("In this blog post I want to add some ideas around libraries in WASM.");

            p
        });

        d
    });
    body.add_element(footer::build());

    CustomDocument::new(Document::new(head, body))
}
