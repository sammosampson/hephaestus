use std::sync::mpsc::*;

use crate::compilation::*;
use crate::acting::*;

pub fn create_test_message_receiver_actor() -> (CompilationActorHandle, Receiver<CompilationMessage>) {
    let (sender, receiver) = channel::<CompilationMessage>();
    (create_handle(sender), receiver)
}