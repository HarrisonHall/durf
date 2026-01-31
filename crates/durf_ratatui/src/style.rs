/// Style for changing how the widget is rendered.
#[derive(Debug, Clone, Default)]
pub struct DurfWidgetStyle {
    pub general: GeneralStyle,
}

/// General style for elements.
#[derive(Debug, Clone, Default)]
pub struct GeneralStyle {
    pub section: SectionStyle,
    pub text: TextStyle,
}

/// Section style.
#[derive(Debug, Clone, Default)]
pub struct SectionStyle {
    pub border: BorderStyle,
}

/// Section border style.
#[derive(Debug, Clone, Default)]
pub enum BorderStyle {
    /// Do not show a border.
    None,
    /// Indent the section.
    Indent,
    /// Provide spacing after the section.
    Spacing,
    /// Show a full border around the section.
    #[default]
    FullBorder,
}

/// Text style.
#[derive(Debug, Clone, Default)]
pub struct TextStyle {}
