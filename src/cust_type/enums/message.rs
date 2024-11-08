#[derive(Debug)]
pub(super) enum Message {
    Quit(String, String),
    Start { x: i32, y: i32 },
    Write(String),
}