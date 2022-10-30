use crate::{
    parsing::*,
    typing::*
};

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

pub fn parse_struct_instance_access(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNode {
    let instance = create_node(
        instance_item(name, unresolved_resolvable_type(), unknown_scope()),
        position
    );
    let member = parse_struct_instance_member(lexer);
    create_node(member_expr_item(instance, member, unresolved_resolvable_type()), position)
}

pub fn parse_struct_instance_member(lexer: &mut Lexer) -> AbstractSyntaxNode {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Identifier(name) =>
            create_node(member_item(name, unresolved_resolvable_type()), token.position),
        _ => create_error_node(expected_identifier_error(), token.position),
    }
}

pub fn member_expr_item(
    instance: AbstractSyntaxNode,
    member: AbstractSyntaxNode,
    member_expression_type: ResolvableType
) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::MemberExpr {
        instance,
        member,
        member_expression_type
    }
}

pub fn member_item(
    name: String,
    member_type: ResolvableType
) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Member {
        name,
        member_type
    }
}

pub fn instance_item(
    name: String,
    instance_type: ResolvableType,
    scope: Scope
) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Instance {
        name,
        instance_type,
        scope
    }
}