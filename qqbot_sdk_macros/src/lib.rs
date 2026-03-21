use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    ItemFn, LitStr, Result,
};

struct CommandArgs {
    prefix: LitStr,
}

impl Parse for CommandArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        // let method: Option<Ident> = input.parse()?;
        // input.parse::<Token![,]>()?;
        let path: LitStr = input.parse()?;

        Ok(CommandArgs {  prefix: path })
    }
}

#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    let CommandArgs {  prefix } = parse_macro_input!(args as CommandArgs);
    let func = parse_macro_input!(input as ItemFn);
    let fn_name = &func.sig.ident;



    let expanded = quote! {
        #func

        inventory::submit! {
            qqbot_sdk::CommandDef {
                prefix: #prefix,
                handler: #fn_name
            }
        }
    };

    expanded.into()
}