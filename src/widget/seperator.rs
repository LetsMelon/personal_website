use std::sync::atomic::{AtomicBool, Ordering};

use html_site_generator::attributes::SetHtmlAttributes;
use html_site_generator::html::div::Div;
use html_site_generator::html::IntoHtmlNode;

#[derive(Debug)]
pub struct Separator;

impl IntoHtmlNode for Separator {
    fn transform_into_raw_html(&self, buffer: &mut dyn std::io::Write) -> anyhow::Result<()> {
        let mut d = Div::new();
        d.add_class("separator");

        d.transform_into_raw_html(buffer)?;

        Ok(())
    }

    fn transform_into_raw_css(&self, buffer: &mut dyn std::io::Write) -> anyhow::Result<()> {
        static WROTE_CSS: AtomicBool = AtomicBool::new(false);

        if !WROTE_CSS.load(Ordering::Relaxed) {
            writeln!(
                buffer,
                ".separator {{
    border-top: 1.5px solid #000000;
    margin-top: 5px;
    margin-bottom: 10px;
}}"
            )?;

            WROTE_CSS.store(true, Ordering::Relaxed);
        }

        Ok(())
    }

    fn transform_into_raw_js(&self, _buffer: &mut dyn std::io::Write) -> anyhow::Result<()> {
        Ok(())
    }
}
