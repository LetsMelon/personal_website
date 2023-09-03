use html_site_generator::html::document::Document;
use html_site_generator::html::IntoHtmlNode;

#[derive(Debug)]
pub struct CustomDocument {
    inner: Document,
}

impl CustomDocument {
    pub fn new(document: Document) -> Self {
        CustomDocument { inner: document }
    }
}

impl IntoHtmlNode for CustomDocument {
    fn transform_into_raw_html(&self, buffer: &mut dyn std::io::Write) -> anyhow::Result<()> {
        self.inner.transform_into_raw_html(buffer)?;

        Ok(())
    }

    fn transform_into_raw_css(&self, buffer: &mut dyn std::io::Write) -> anyhow::Result<()> {
        writeln!(
            buffer,
            ":root {{
    --text: #060605;
    --background: #edede9;
    --primary: #7a7ad6;
    --secondary: #d1d1f5;
    --accent: #7d2dcd;
}}

@font-face {{
    font-family: \"Space Mono\";
    src: url(\"../fonts/SpaceMono-Regular.ttf\"),
    url(\"../fonts/SpaceMono-Bold.ttf\"),
    url(\"../fonts/SpaceMono-BoldItalic.ttf\"),
    url(\"../fonts/SpaceMono-Italic.ttf\");
}}

body {{
    background-color: var(--background);
    font-family: \"Space Mono\";
}}

#rust-lang-icon {{
    height: 1em;
    position: relative;
    top: 0.125em;
    left: 0.125em;
}}
"
        )?;

        self.inner.transform_into_raw_css(buffer)?;

        Ok(())
    }

    fn transform_into_raw_js(&self, buffer: &mut dyn std::io::Write) -> anyhow::Result<()> {
        self.inner.transform_into_raw_js(buffer)?;

        Ok(())
    }
}
