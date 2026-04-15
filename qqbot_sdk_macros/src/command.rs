use crate::context_injection;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parse, parse::ParseStream, FnArg, ItemFn, LitStr, Result};
use crate::context_injection::quoting_context_param;

/// Command 宏的参数
///
/// 只包含命令前缀，不需要指定状态类型
pub struct CommandArgs {
    /// 命令前缀（如 "/ping"）
    pub prefix: LitStr,
}

impl Parse for CommandArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let prefix: LitStr = input.parse()?;
        Ok(CommandArgs { prefix })
    }
}

/// Command 宏的实现函数
///
/// 这个函数会：
/// 1. 调用依赖注入模块分析函数参数
/// 2. 生成包装函数，处理依赖注入
/// 3. 生成 inventory 注册代码
///
/// # 参数
/// * `args` - 宏参数（命令前缀）
/// * `func` - 被标注的函数
///
/// # 返回
/// 生成的完整代码
pub fn command_impl(args: CommandArgs, func: ItemFn) -> TokenStream {
    let prefix = args.prefix;
    let fn_name = &func.sig.ident;
    // 生成包装函数的名称
    let wrapper_name = format_ident!("__qqbot_sdk_command_wrapper_{}", fn_name);
    let is_async = func.sig.asyncness.is_some();

    let mut param_extractions = Vec::new();
    let mut call_args = Vec::new();

    for (index, arg) in func.sig.inputs.iter().enumerate() {
        match arg {
            FnArg::Receiver(_) => {
                panic!("Methods with self are not supported");
            }
            FnArg::Typed(typed) => {
                let ty = &typed.ty;
                // 如果是Context
                if let Some(inner_type) = context_injection::extract_context_inner_type(ty) {
                    call_args.push(quoting_context_param(inner_type));
                } else {
                    // 从消息体转换参数
                    let arg_name = format_ident!("__arg_{}", index);
                    param_extractions.push(quote! {
                        let #arg_name: #ty = <#ty as qqbot_sdk::FromCommonMessage<'_>>::from(__message);
                    });
                    call_args.push(quote! { #arg_name });
                }
            }
        }
    }

    // 判断是否异步生成对应封装
    let invoke = if is_async {
        quote! {
            let result = #fn_name(#(#call_args),*).await;
        }
    } else {
        quote! {
            let result = #fn_name(#(#call_args),*);
        }
    };

    // 生成最终代码
    quote! {
        // 保留原函数定义
        #func

        // 在匿名常量中生成包装代码，避免命名冲突
        const _: () = {
            // 包装函数：接收消息和容器，返回 Future
            fn #wrapper_name<'a>(
                __message: &'a dyn qqbot_sdk::CommonMessage,
                __store: &'a qqbot_sdk::ContextStore,
            ) -> qqbot_sdk::CommandHandleFuture<'a> {
                ::std::boxed::Box::pin(async move {
                    #(#param_extractions)*
                    #invoke
                    // 将结果转换为统一的输出格式
                    qqbot_sdk::CommandOutput::into_output(result)
                })
            }

            // 使用 inventory 注册命令
            qqbot_sdk::inventory::submit! {
                qqbot_sdk::CommandDef {
                    prefix: #prefix,
                    handler: #wrapper_name
                }
            }
        };
    }
}
