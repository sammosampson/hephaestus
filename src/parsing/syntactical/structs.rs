use crate::{
    parsing::*,
    types::*
};

pub fn struct_item(
    name: String,
    fields: AbstractSyntaxChildNodes
) -> AbstractSyntaxNodeItem {
    AbstractSyntaxNodeItem::Struct { name, fields }
}

pub fn parse_struct(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNodeResult {

    if !is_open_brace(&peek_next_token(lexer).item) {
        // no brace not supported yet
        return Err(create_error(unimplemented_error(), position));
    }

    eat_next_token(lexer);

    let fields = parse_struct_fields(lexer)?;
    
    assert!(is_close_brace(&peek_next_token(lexer).item));
    eat_next_token(lexer);


    Ok(create_node(struct_item(name, fields), position))
}

fn parse_struct_fields(lexer: &mut Lexer) -> AbstractSyntaxChildNodesResult {
    let mut fields = vec!();
    
    loop {
        if is_close_brace(&peek_next_token(lexer).item) {
            return Ok(fields)
        }
    
        fields.push(parse_declaration(lexer)?);

        let next_token = peek_next_token(lexer);
        
        if is_close_brace(&next_token.item) {
            return Ok(fields)
        }

        if is_line_terminiator(&next_token.item) {
            eat_next_token(lexer);
        } else {
            return Err(create_error(expected_line_terminator_error(), next_token.position));
        }
    }
}

pub fn parse_struct_instance_access(name: String, lexer: &mut Lexer, position: SourceFilePosition) -> AbstractSyntaxNodeResult {
    let instance = create_node(
        instance_item(name, unresolved_resolvable_type(), unknown_scope()),
        position
    );
    let member = parse_struct_instance_member(lexer)?;
    Ok(create_node(member_expr_item(instance, member, unresolved_resolvable_type()), position))
}

pub fn parse_struct_instance_member(lexer: &mut Lexer) -> AbstractSyntaxNodeResult {
    let token = get_next_token(lexer);

    match token.item {
        SourceTokenItem::Identifier(name) =>
            Ok(create_node(member_item(name, unresolved_resolvable_type()), token.position)),
        _ => Err(create_error(expected_identifier_error(), token.position)),
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

pub fn try_get_member_name(item: &AbstractSyntaxNodeItem) -> Option<&str> {
    if let AbstractSyntaxNodeItem::Member { name, .. } = item {
        return Some(name);
    }
    None
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

pub fn try_get_instance_name_and_type(item: &AbstractSyntaxNodeItem) -> Option<(&str, &ResolvableType)> {
    if let AbstractSyntaxNodeItem::Instance { name, instance_type, .. } = item {
        return Some((name, instance_type));
    }
    None
}