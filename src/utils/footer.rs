use html_site_generator::attributes::SetHtmlAttributes;
use html_site_generator::html::address::Address;
use html_site_generator::html::div::Div;
use html_site_generator::html::footer::Footer;
use html_site_generator::html::hyperlink::HyperlinkBuilder;
use html_site_generator::html::list::{List, ListType};
use html_site_generator::html::paragraph::Paragraph;

pub fn build() -> Footer {
    let mut f = Footer::new();

    f.add_element({
        let mut d = Div::new();
        d.add_class("separator");
        d
    });

    f.add_element("Further links:");

    f.add_element({
        let mut l = List::new_with_ordering(ListType::Unordered);

        l.add_element({
            let mut h = HyperlinkBuilder::default()
                .href("https://github.com/LetsMelon")
                .build()
                .unwrap();
            h.add_element("Github profile");
            h
        });

        l
    });

    f.add_element({
        let mut a = Address::new();

        a.add_element({
            let mut p = Paragraph::new();

            p.add_element("Created by ");
            p.add_element({
                let mut h = HyperlinkBuilder::default()
                    .href("mailto:domi.m@outlook.com")
                    .rel(None)
                    .target(None)
                    .build()
                    .unwrap();

                h.add_element("Domenic Melcher");

                h
            });
            p.add_element(" with the help of my project ");
            p.add_element({
                let mut h = HyperlinkBuilder::default()
                    .href("https://github.com/LetsMelon/html-site-generator")
                    .build()
                    .unwrap();

                h.add_element("html-site-generator");

                h
            });
            p.add_element(".");

            p
        });

        a
    });

    f
}
