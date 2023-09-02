use std::io::Write;

use anyhow::Result;
use html_site_generator::html::div::Div;
use html_site_generator::html::hyperlink::HyperlinkBuilder;
use html_site_generator::html::line_break::LineBreak;
use html_site_generator::html::paragraph::Paragraph;
use html_site_generator::html::text::{TextElement, TextElementStyling};
use html_site_generator::html::{IntoHtmlNode, IsParagraph};

#[derive(Debug, Clone)]
pub struct Project {
    title: String,
    description: String,
    link: Option<String>,
    languages: Vec<String>,
}

impl Project {
    pub fn new<S: Into<String>, D: IsParagraph>(
        title: S,
        description: D,
        link: Option<String>,
        languages: Vec<String>,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.to_raw(),
            link,
            languages,
        }
    }
}

impl IntoHtmlNode for Project {
    fn transform_into_html_node(&self, buffer: &mut dyn Write) -> Result<()> {
        let mut d = Div::new();

        let mut p = Paragraph::new();

        p.add_element(TextElement::new_with_styling(
            self.title.clone(),
            TextElementStyling::Bold,
        ));

        p.add_element(LineBreak::new());
        p.add_element(format!("Languages: {}", self.languages.join(", ")));

        let mut raw_link_node = "Link: ".to_string();
        if let Some(link) = &self.link {
            let mut h = HyperlinkBuilder::default().href(link).build().unwrap();

            h.add_element("Github");

            raw_link_node.push_str(&h.to_raw());
        } else {
            raw_link_node.push_str("not public");
        }
        p.add_element(LineBreak::new());
        p.add_element(raw_link_node);

        p.add_element(LineBreak::new());

        p.add_element(self.description.to_raw());

        d.add_element(p);

        d.transform_into_html_node(buffer)?;

        Ok(())
    }
}
