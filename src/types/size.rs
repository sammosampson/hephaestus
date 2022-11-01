use crate::types::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum TypeSize {
    NotRequired,
    Resolved { size_in_bytes: usize },
}

pub fn try_get_resolved_type_size(type_size: &TypeSize) -> Option<usize> {
    if let TypeSize::Resolved { size_in_bytes } = type_size {
        return Some(*size_in_bytes);
    }
    None
}

pub fn resolved_type_size(size_in_bytes: usize) -> TypeSize {
    TypeSize::Resolved { size_in_bytes }
}

pub fn not_required_type_size() -> TypeSize {
    TypeSize::NotRequired
}

pub fn try_get_type_size_from_resolvable_type(variable_type: &ResolvableType) -> Option<usize> {
    if let Some(runtime_type) = try_get_resolved_runtime_type_pointer(variable_type) {
        if let Some(type_size) = try_get_resolved_type_size(&runtime_type.size) {
            return Some(type_size);
        }
    }
    None
}