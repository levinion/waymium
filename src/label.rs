#[derive(Clone)]
pub struct Label {
    pub matched: gtk4::Label,
    pub unmatched: gtk4::Label,
}

impl Label {
    pub fn new(hint: impl AsRef<str>) -> Self {
        Self {
            matched: gtk4::Label::builder().css_classes(["hint-matched"]).build(),
            unmatched: gtk4::Label::builder()
                .label(hint.as_ref())
                .css_classes(["hint-unmatched"])
                .build(),
        }
    }

    pub fn text(&self) -> String {
        self.matched.text().to_string() + self.unmatched.text().as_str()
    }
}
