use crate::parsing::*;
use crate::typing::*;

pub fn parse_top_level_declaration(filename: String, name: String, lexer: &mut Lexer, position: SourceFilePosition, units: &mut CompilationUnits) -> AbstractSyntaxNode {
    if is_struct_keyword(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_struct(name, lexer, position)
    }

    if is_open_paren(&peek_next_token(lexer).item) {
        eat_next_token(lexer);
        return parse_procedure_header(filename, name, lexer, position, units)
    }
    
    parse_inferred_constant(name, lexer, position)
}

pub fn parse_known_type_top_level_declaration(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    if let Some(resolvable_type) = try_parse_type(lexer) {
        eat_next_token(lexer);
        if is_initialise_assignment( &peek_next_token(lexer).item) {
            eat_next_token(lexer);
            return parse_constant(name, lexer, position, resolvable_type);
        }        
        return create_error_node(expected_initialise_assignment_error(), get_next_token(lexer).position);
    }
    create_error_node(expected_type_error(), get_next_token(lexer).position)
}

pub fn parse_declaration(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let name_token = peek_next_token(lexer);
    if let Some(name) = try_get_identifier(name_token.item) {
        eat_next_token(lexer);
        
        if is_initialise_assignment(&peek_next_token(lexer).item) {
            eat_next_token(lexer);

            if let Some(arg_type) = try_parse_type(lexer) {
                eat_next_token(lexer);
                return create_node(member_declaration_item(name, arg_type), name_token.position)
            }

            return create_error_node(unimplemented_error(), peek_next_token(lexer).position);        
        }

        return create_error_node(expected_initialise_assignment_error(), peek_next_token(lexer).position);        
    }
    
    create_error_node(expected_declaration_name_error(), peek_next_token(lexer).position)     
}

pub fn member_declaration_item(name: String, type_id: ResolvableType) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::MemberDeclaration { name, member_type: type_id }
}