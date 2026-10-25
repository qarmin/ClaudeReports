use muda::Menu;

fn main() {
    let _ = <muda::Menu as muda::ContextMenu>::gtk_context_menu(&Menu::new());
}
