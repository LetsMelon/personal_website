use html_site_generator::attributes::{HtmlAttributesBuilder, SetHtmlAttributes};
use html_site_generator::html::div::Div;
use html_site_generator::html::hyperlink::HyperlinkBuilder;
use html_site_generator::html::list::{List, ListType};

pub fn build() -> Div {
    let mut d = Div::new();

    let mut l = List::new_with_ordering(ListType::Unordered);
    l.add_class("navbar");

    l.add_element_with_attributes(
        {
            let mut h = HyperlinkBuilder::default()
                .rel(None)
                .target(None)
                .href("/")
                .build()
                .unwrap();
            h.add_element("Home");
            h
        },
        HtmlAttributesBuilder::default()
            .class(vec!["navbar_item".to_string()])
            .build()
            .unwrap(),
    );

    l.add_element_with_attributes(
        {
            let mut h = HyperlinkBuilder::default()
                .rel(None)
                .target(None)
                .href("/blogs")
                .build()
                .unwrap();
            h.add_element("Blogs 🚧");
            h
        },
        HtmlAttributesBuilder::default()
            .class(vec!["navbar_item".to_string()])
            .build()
            .unwrap(),
    );

    d.add_element(l);

    d.add_element({
        let mut d = Div::new();
        d.add_class("separator");
        d
    });

    d
}
