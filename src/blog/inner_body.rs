use html_site_generator::attributes::SetHtmlAttributes;
use html_site_generator::html::div::Div;
use html_site_generator::html::line_break::LineBreak;
use html_site_generator::html::paragraph::Paragraph;
use html_site_generator::html::text::{TextElement, TextElementStyling};

pub fn build() -> Div {
    let mut d = Div::new();
    d.set_id("inner_body");

    d.add_element({
        let mut p = Paragraph::new();

        // TODO `<h1>`
        p.add_element(TextElement::new_with_styling(
            "🚧 Blogs 🚧",
            TextElementStyling::Bold,
        ));

        p
    });

    d.add_element({
        let mut p = Paragraph::new();

        p.add_element("There should be some blog posts, but they are still under construction.");
        p.add_element(LineBreak::new());
        p.add_element("Either you can wait until only the first blog is online, or you will just have to come at another time.");

        p
    });

    d
}
