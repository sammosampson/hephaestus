use std::sync::mpsc::*;
use crate::{
    compilation::*,
    tests::file_system::*
};

pub fn compile_and_get_message_receiver(file_path: &str, file_reader: MockFileReader) -> Receiver<CompilationMessage> {    
    let (message_sender, message_receiver) = channel::<CompilationMessage>();
    let message_wire_tap = create_send_message_wire_tap(message_sender);
    compile(file_path.to_string(), file_reader, message_wire_tap);
    message_receiver
}

pub struct SendMessageWireTap {
    sender: Sender<CompilationMessage>
}

fn create_send_message_wire_tap(sender: Sender<CompilationMessage>) -> SendMessageWireTap {
    SendMessageWireTap { sender }
}

impl WireTapCompilationMessage for SendMessageWireTap {
    fn tap(&mut self, message: &CompilationMessage) {
        self.sender.send(message.clone()).unwrap();
    }
}