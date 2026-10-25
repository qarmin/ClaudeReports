use x11rb_protocol::connection::Connection;

fn main() {
    let mut connection = Connection::default();
    connection.enqueue_packet(vec![]);
}
