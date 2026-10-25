use alsa::seq::Event;
use alsa::seq::EventType;

fn main() {
    let v = ();
    let _ = Event::new::<()>(EventType::Pgmchange, &v);
}
