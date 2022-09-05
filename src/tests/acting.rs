use std::sync::mpsc::*;

use crate::acting::*;
use crate::compilation::*;

pub fn create_test_message_receiver_handle() -> (CompilationActorHandle, Receiver<CompilationMessage>) {
    let (sender, receiver) = channel::<CompilationMessage>();
    (create_handle(sender), receiver)
}