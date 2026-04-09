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
    // 参数和函数的信息解析
    let CommandArgs { prefix } = parse_macro_input!(args as CommandArgs);
    let func = parse_macro_input!(input as ItemFn);
    let is_async = func.sig.asyncness.is_some();
    let fn_name = &func.sig.ident;
    let wrapper_name = format_ident!("__qqbot_sdk_command_wrapper_{}", fn_name);

    // 函数的形参解析
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
                // 预处理形参的转换
                extracted_args.push(quote! {
                    let #arg_name: #ty = <#ty as qqbot_sdk::FromCommonMessage<'_>>::from(payload);
                });
                call_args.push(quote! { #arg_name });
            }
        }
    }

    // 根据是否异步函数的封装
    let invoke = if is_async {
        quote! {
            let result = #fn_name(#(#call_args),*).await;
        }
    } else {
        quote! {
            let result = #fn_name(#(#call_args),*);
        }
    };

    // 最终的展开结果
    let expanded = quote! {
        #func

        const _: () = {
            fn #wrapper_name<'a>(payload: &'a dyn qqbot_sdk::CommonMessage) -> qqbot_sdk::CommandHandleFuture<'a> {
                ::std::boxed::Box::pin(async move {
                    #(#extracted_args)*
                    #invoke
                    qqbot_sdk::CommandOutput::into_output(result)
                })
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
