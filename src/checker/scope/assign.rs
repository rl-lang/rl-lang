use crate::{
    ast::statements::TypeAnnotation,
    checker::structs::{CheckType, TypeChecker},
    utils::{span::Span, suggest::closest_match},
};

impl TypeChecker {
    // search for variable name via loop thru the scopes
    // and if it is found then checks
    // if constant add error to const_error
    // if variable but type mismatch add error to type_error
    // then break the loop and push the error
    // if not found pushs undefined variable error
    pub fn assign(&mut self, name: &str, value_type: CheckType, span: Span) {
        let mut const_error: Option<String> = None;
        let mut type_error: Option<String> = None;
        let mut found = false;

        for scope in self.scopes.iter_mut().rev() {
            if let Some(item) = scope.get_mut(name) {
                found = true;
                let widens = matches!(
                    (&item.type_annotation, &value_type),
                    (
                        CheckType::Known(TypeAnnotation::Int | TypeAnnotation::CInt),
                        CheckType::Known(TypeAnnotation::Byte | TypeAnnotation::CByte)
                    )
                );
                if item.is_const {
                    const_error = Some(format!("cannot assign to constant '{}'", name));
                } else if !widens
                    && !value_type.matches(&item.type_annotation)
                    && !value_type.is_null()
                {
                    type_error = Some(format!(
                        "cannot assign {} to variable '{}' declared as {}",
                        value_type.info(),
                        name,
                        item.type_annotation.info(),
                    ));
                }
                break;
            }
        }

        if let Some(msg) = const_error.or(type_error) {
            self.error(msg, span);
            return;
        }

        if !found {
            let all_keys: Vec<String> = self
                .scopes
                .iter()
                .flat_map(|s| s.keys().cloned().collect::<Vec<_>>())
                .collect();
            let suggestion = closest_match(name, all_keys.iter().map(|s| s.as_str()));

            self.error_with_help(format!("undefined variable '{}'", name), span, suggestion);
        }
    }
}
