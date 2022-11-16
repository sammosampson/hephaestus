
use crate::types::*;

#[derive(PartialEq, Debug, Clone)]
pub enum ResolvableType {
    Resolved(RuntimeTypePointer),
    UnresolvedNamed(String),
    Unresolved
}

pub fn unresolved_resolvable_type() -> ResolvableType {
    ResolvableType::Unresolved
}

pub fn unresolved_named_resolvable_type(name: String) -> ResolvableType {
    ResolvableType::UnresolvedNamed(name)
}

pub fn resolved_resolvable_type(type_pointer: RuntimeTypePointer) -> ResolvableType {
    ResolvableType::Resolved(type_pointer)
}

pub fn try_get_instance_member_offset(instance_type: &ResolvableType, member_name: &str) -> Option<usize> {
    if let Some(pointer) = try_get_resolved_runtime_type_pointer(instance_type) {
        return match &pointer.item {
            RuntimeTypeItem::String { members } => Some(get_member_offset(members, member_name)),
            //RuntimeTypeItem::Struct { members } => get_member_offset(members, member_name),
            _ => None
        }
    }
    None
}

fn get_member_offset(members: &RuntimeTypeMembers, member_name: &str) -> usize {
    let mut offset = 0;
    for member in members {
        if member.name == member_name {
            break;
        }
        
        if let Some(type_size) = try_get_resolved_type_size(&member.field_type.size) {
            offset += type_size
        }
    }
    offset
}

pub fn try_get_resolved_runtime_type_pointer(resolvable_type: &ResolvableType) -> OptionalRuntimeTypePointer {
    if let ResolvableType::Resolved(pointer) = resolvable_type {
       return Some(pointer.clone());
    }
    None
}