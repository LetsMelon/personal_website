use html_site_generator::html::head::Head;
use html_site_generator::html::link::{LinkBuilder, Relationship};
use html_site_generator::html::meta::Meta;
use html_site_generator::html::title::Title;

pub fn build() -> Head {
    let mut h = Head::new();

    h.add_element({
        let mut m = Meta::new();

        m.add_pair("charset", "utf-8");

        m
    });
    h.add_element({
        let mut m = Meta::new();

        m.add_pair("name", "viewport");
        m.add_pair("content", "width=device-width, initial-scale=1");

        m
    });

    h.add_element(Title::new("Domenic Melcher Website"));

    h.add_element(
        LinkBuilder::default()
            .rel(Relationship::Stylesheet)
            .media_type("text/css")
            .href("styles.css")
            .build()
            .unwrap(),
    );

    h
}
