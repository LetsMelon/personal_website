use std::io::Write;

use anyhow::Result;
use html_site_generator::attributes::SetHtmlAttributes;
use html_site_generator::html::div::Div;
use html_site_generator::html::hyperlink::HyperlinkBuilder;
use html_site_generator::html::image::ImageBuilder;
use html_site_generator::html::line_break::LineBreak;
use html_site_generator::html::list::{List, ListType};
use html_site_generator::html::paragraph::Paragraph;
use html_site_generator::html::text::{TextElement, TextElementStyling};
use html_site_generator::html::{IntoHtmlNode, IsParagraph};

#[derive(Debug, Clone)]
struct Project {
    title: String,
    description: String,
    link: Option<String>,
    languages: Vec<String>,
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

        p.add_element(self.description.clone());

        d.add_element(p);

        d.transform_into_html_node(buffer)?;

        Ok(())
    }
}

pub fn build() -> Div {
    let mut inner_body_div = Div::new();
    inner_body_div.set_id("inner_body");

    inner_body_div.add_element({
        let mut p = Paragraph::new();

        p.add_element("Hi, my name is ");
        p.add_element(TextElement::new_with_styling(
            "Domenic Melcher",
            TextElementStyling::Bold,
        ));
        p.add_element("!");

        p
    });

    inner_body_div.add_element({
        let mut p = Paragraph::new();

        // TODO add element `span` for the age
        p.add_element("I'm a 20-year-old student with a passion for coding. Currently, I'm pursuing a bachelor's degree in Software & Information Engineering at the ");

        p.add_element({
            let mut h = HyperlinkBuilder::default()
                .href("https://www.tuwien.at/")
                .build()
                .unwrap();

            h.add_element("Technical University of Vienna");

            h
        });

        p.add_element("in Austria.");

        p
    });

    inner_body_div.add_element({
        let mut p = Paragraph::new();

        p.add_element(
            "Since mid 2020 my main programming language of choice for hobby projects is Rust",
        );
        p.add_element(
            {let mut i = ImageBuilder::default()
            .src("/assets/ferris.svg")
            .alt("Rust logo")
            .build()
            .unwrap(); i.set_id("rust-lang-icon"); i}
        );
        p.add_element(
            ", but I have also experience in JavaScript, Python, Java and some other programming languages.",
        );

        p
    });

    inner_body_div.add_element("Some of my projects are:");

    inner_body_div.add_element({
        let mut l = List::new_with_ordering(ListType::Unordered);

        let projects = vec![
            Project {
                title: "rusvid".to_string(),
                description: "Library to create animations out of svg paths.".to_string(),
                link: Some("https://www.github.com/letsmelon/rusvid".to_string()),
                languages: vec!["Rust".to_string()],
            },
            Project {
                title: "tsql".to_string(),
                description: "Custom scripting language that can be compiled down to sql."
                    .to_string(),
                link: Some("https://www.github.com/letsmelon/tsql".to_string()),
                languages: vec!["Rust".to_string()],
            },
            Project {
                title: "how_many_days_until".to_string(),
                description: "Small cli tool count the days between two days.".to_string(),
                link: Some("https://www.github.com/letsmelon/how_many_days_until".to_string()),
                languages: vec!["Rust".to_string()],
            },
            Project {
                title: "adanui".to_string(),
                description: "TODO".to_string(),
                link: None,
                languages: vec!["JavaScript".to_string(), "Docker".to_string()],
            },
        ];

        for project in &projects {
            l.add_element(project.clone());
        }

        l
    });

    inner_body_div
}
