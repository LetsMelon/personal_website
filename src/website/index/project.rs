use std::io::Write;

use html_site_generator::html::div::Div;
use html_site_generator::html::hyperlink::HyperlinkBuilder;
use html_site_generator::html::line_break::LineBreak;
use html_site_generator::html::paragraph::Paragraph;
use html_site_generator::html::text::{TextElement, TextElementStyling};
use html_site_generator::html::{IntoHtmlNode, IntoHtmlNodeResult, IsParagraph};

#[derive(Debug)]
pub struct Project {
    inner: Div,
}

impl Project {
    pub fn new<S: Into<String>, D: IsParagraph>(
        title: S,
        description: D,
        link: Option<String>,
        languages: Vec<String>,
    ) -> Self {
        let mut d = Div::new();

        let mut p = Paragraph::new();

        p.add_element(TextElement::new_with_styling(
            title.into(),
            TextElementStyling::Bold,
        ));

        p.add_element(LineBreak::new());
        p.add_element(format!("Languages: {}", languages.join(", ")));

        let mut raw_link_node = "Link: ".to_string();
        if let Some(link) = link {
            let mut h = HyperlinkBuilder::default().href(link).build().unwrap();

            h.add_element("Github");

            raw_link_node.push_str(&h.to_raw());
        } else {
            raw_link_node.push_str("not public");
        }
        p.add_element(LineBreak::new());
        p.add_element(raw_link_node);

        p.add_element(LineBreak::new());

        p.add_element(description.to_raw());

        d.add_element(p);

        Self { inner: d }
    }
}

impl IntoHtmlNode for Project {
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        self.inner.transform_into_raw_html(buffer)?;

        Ok(())
    }

    fn transform_into_raw_css(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        self.inner.transform_into_raw_css(buffer)?;

        Ok(())
    }

    fn transform_into_raw_js(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        self.inner.transform_into_raw_js(buffer)?;

        Ok(())
    }
}
