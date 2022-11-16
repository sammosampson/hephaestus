use std::sync::mpsc::*;

use crate::compilation::*;
use crate::acting::*;
use crate::utilities::*;

pub fn create_test_message_receiver_actor() -> (CompilationActorHandle, Receiver<CompilationMessage>) {
    let (sender, receiver) = channel::<CompilationMessage>();
    (create_handle(sender, string("test_message_receiver_actor")), receiver)
}