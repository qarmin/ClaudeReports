use krilla::action::Action;
use krilla::action::ResetFormAction;
use krilla::annotation::Annotation;
use krilla::annotation::LinkAnnotation;
use krilla::annotation::Target;
use krilla::document::Document;
use krilla::geom::Rect;

fn main() {
    let rect = Rect::from(Rect::from_xywh(0.0f32, 0.0f32, 0.0f32, 0.0f32).unwrap());
    let action = Action::from(ResetFormAction::All);
    let mut document = Document::new();
    let mut page = document.start_page();
    let mut form_field = <krilla::form::FormField<u32> as ::core::default::Default>::default();
    let _ = page.add_widget_annotation::<u32>(&mut form_field, Annotation::from(LinkAnnotation::new(rect, Target::Action(action.clone()))));
}
