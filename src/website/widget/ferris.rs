use std::sync::atomic::{AtomicBool, Ordering};

use html_site_generator::attributes::SetHtmlAttributes;
use html_site_generator::html::image::{Image, ImageBuilder};
use html_site_generator::html::{IntoHtmlNode, IntoHtmlNodeResult};

#[derive(Debug)]
pub struct Ferris {
    inner: Image,
}

impl Ferris {
    pub fn new() -> Self {
        let mut i = ImageBuilder::default()
            .src("/assets/ferris.svg")
            .alt("Rust logo")
            .build()
            .unwrap();
        i.add_id("rust-lang-icon");

        Ferris { inner: i }
    }
}

impl IntoHtmlNode for Ferris {
    fn transform_into_raw_html(&self, buffer: &mut dyn std::io::Write) -> IntoHtmlNodeResult<()> {
        self.inner.transform_into_raw_html(buffer)?;

        Ok(())
    }

    fn transform_into_raw_css(&self, buffer: &mut dyn std::io::Write) -> IntoHtmlNodeResult<()> {
        self.inner.transform_into_raw_css(buffer)?;

        // only write the css once, this can also be created with an unique link
        static WROTE_CSS: AtomicBool = AtomicBool::new(false);
        if !WROTE_CSS.load(Ordering::Relaxed) {
            writeln!(
                buffer,
                "#rust-lang-icon {{
    height: 1em;
    position: relative;
    top: 0.125em;
    left: 0.125em;
}}"
            )?;

            WROTE_CSS.store(true, Ordering::Relaxed);
        }

        Ok(())
    }

    fn transform_into_raw_js(&self, buffer: &mut dyn std::io::Write) -> IntoHtmlNodeResult<()> {
        self.inner.transform_into_raw_js(buffer)?;

        Ok(())
    }
}
