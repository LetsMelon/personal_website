use html_site_generator::attributes::{HtmlAttributesBuilder, SetHtmlAttributes};
use html_site_generator::html::hyperlink::HyperlinkBuilder;
use html_site_generator::html::list::{List, ListType};

pub fn build() -> List {
    let mut l = List::new_with_ordering(ListType::Unordered);
    l.set_class("navbar");

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
            .class("navbar_item")
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
            .class("navbar_item")
            .build()
            .unwrap(),
    );

    l
}
