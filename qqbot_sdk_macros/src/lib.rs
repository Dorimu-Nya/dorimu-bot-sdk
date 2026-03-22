use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, FnArg, ItemFn, LitStr, Result,
};

struct CommandArgs {
    prefix: LitStr,
}

impl Parse for CommandArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let path: LitStr = input.parse()?;
        Ok(CommandArgs { prefix: path })
    }
}

#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    let CommandArgs { prefix } = parse_macro_input!(args as CommandArgs);
    let func = parse_macro_input!(input as ItemFn);
    let fn_name = &func.sig.ident;
    let wrapper_name = format_ident!("__qqbot_sdk_command_wrapper_{}", fn_name);

    let mut extracted_args = Vec::new();
    let mut call_args = Vec::new();
    for (index, arg) in func.sig.inputs.iter().enumerate() {
        match arg {
            FnArg::Receiver(receiver) => {
                return syn::Error::new_spanned(
                    receiver,
                    "#[command] only supports free functions",
                )
                .to_compile_error()
                .into();
            }
            FnArg::Typed(typed) => {
                let ty = &typed.ty;
                let arg_name = format_ident!("__qqbot_sdk_arg_{index}");
                extracted_args.push(quote! {
                    let #arg_name: #ty = <#ty as qqbot_sdk::FromCommonMessage<'_>>::from(payload);
                });
                call_args.push(quote! { #arg_name });
            }
        }
    }

    let expanded = quote! {
    #func

    const _: () = {
        fn #wrapper_name(payload: &dyn qqbot_sdk::CommonMessage) {
            #(#extracted_args)*
            #fn_name(#(#call_args),*);
        }

        qqbot_sdk::inventory::submit! {
            qqbot_sdk::CommandDef {
                prefix: #prefix,
                handler: #wrapper_name
            }
        }
    };
};

    expanded.into()
}
