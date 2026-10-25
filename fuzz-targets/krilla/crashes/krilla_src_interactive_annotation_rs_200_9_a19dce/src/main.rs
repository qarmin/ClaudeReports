use krilla::action::Action;
use krilla::action::ResetFormAction;
use krilla::annotation::LinkAnnotation;
use krilla::annotation::Target;

fn main() {
    let _ = LinkAnnotation::new_with_quad_points(vec![], Target::Action(Action::ResetForm(ResetFormAction::Include(vec![]))));
}
