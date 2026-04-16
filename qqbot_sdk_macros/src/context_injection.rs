use quote::quote;
use syn::{GenericArgument, PathArguments, Type, TypePath};

/// 识别 是否 Context<T> 形参并返回 T。
pub fn extract_context_inner_type(ty: &Type) -> Option<Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };
    if !is_context_path(type_path) {
        return None;
    }
    let segment = type_path.path.segments.last()?;
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    let GenericArgument::Type(inner) = args.args.first()? else {
        return None;
    };
    Some(inner.clone())
}

/// 封装 Context形参
pub fn quoting_context_param(inner_type: Type) -> proc_macro2::TokenStream {
    quote! {
        __store.get_context::<#inner_type>()
    }
}

fn is_context_path(type_path: &TypePath) -> bool {
    type_path
        .path
        .segments
        .last()
        .map(|segment| segment.ident == "Context")
        .unwrap_or(false)
}
