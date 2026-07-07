
use handlebars::Handlebars;
use serde_json::Value;

use super::RenderError;

pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(false);
        handlebars.register_escape_fn(handlebars::no_escape);

        Self { handlebars }
    }

    pub fn render(&self, template: &str, data: &Value) -> Result<String, RenderError> {
        self.handlebars
            .render_template(template, data)
            .map_err(|e| RenderError::Template(e.to_string()))
    }

    pub fn register_template(
        &mut self,
        name: &str,
        template: &str,
    ) -> Result<(), RenderError> {
        self.handlebars
            .register_template_string(name, template)
            .map_err(|e| RenderError::Template(e.to_string()))
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}
