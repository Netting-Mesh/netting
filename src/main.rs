mod proto;
use proto::msg::Message;

fn main() {
    let name = String::from("test");
    let msg = build_message(name);
    println!("{:?}", msg);
}

fn build_message(name: String) -> Message {
    let mut msg = Message::new();
    msg.set_body(name);
    msg
}

#[test]
fn build_message_test() {
    let name = String::from("test");
    let msg = build_message(name.clone());
    assert_eq!(msg.get_body(), name);
}
