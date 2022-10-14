use crate::parsing::*;

pub fn struct_item(
    name: String,
    fields: AbstractSyntaxChildNodes
) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Struct { name, fields }
}

pub fn parse_struct(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {

    if !is_open_brace(&peek_next_token(lexer).item) {
        // no brace not supported yet
        return create_error_node(unimplemented_error(), position);
    }

    eat_next_token(lexer);

    let fields = parse_struct_fields(lexer);
    
    assert!(is_close_brace(&peek_next_token(lexer).item));
    eat_next_token(lexer);


    create_node(struct_item(name, fields), position)
}

fn parse_struct_fields(lexer: &mut Lexer) -> AbstractSyntaxChildNodes {
    let mut fields = vec!();
    
    loop {
        if is_close_brace(&peek_next_token(lexer).item) {
            return fields
        }
    
        fields.push(parse_declaration(lexer));

        let next_token = peek_next_token(lexer);
        
        if is_close_brace(&next_token.item) {
            return fields
        }

        if is_line_terminiator(&next_token.item) {
            eat_next_token(lexer);
        } else {
            fields.push(create_error_node(expected_line_terminator_error(), next_token.position));  
            return fields;
        }
    }
}

pub fn parse_struct_instance_field_access(_name: String, _lexer: &mut Lexer, _position: SourceFilePosition) -> AbstractSyntaxNode {
    todo!()
}