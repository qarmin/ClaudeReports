use glutin::config::ConfigTemplateBuilder;

fn main() {
    let config_template_builder = ConfigTemplateBuilder::default();
    let config_template_builder2 = config_template_builder.clone().with_transparency(false);
    let _ = config_template_builder2.clone().with_multisampling(0u8);
}
