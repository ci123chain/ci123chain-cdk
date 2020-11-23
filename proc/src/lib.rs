use hex;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::{abort, proc_macro_error};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::collections::HashSet;
use syn::parse_macro_input;

#[derive(Debug, Default)]
struct ContractFnSig {
    fn_name: String,
    input_type: Vec<String>,
    output_type: Option<String>,
}

static mut CONTRACT_DOC: Vec<ContractFnSig> = vec![];

#[derive(Debug)]
enum CompileError {
    UnexpectedArgType,
    UnexpectedReturnType,
    ExpectedIdentifier,
    ExpectedFunctionArgs,
    DuplicateIdentifier,
}

impl ToString for CompileError {
    fn to_string(&self) -> String {
        match self {
            CompileError::UnexpectedArgType => "expected one of: `bool`,`u32`,`i32`,`u64`,`i64`,`u128`,`i128`,`String`,`&str`,`&[u8]`".to_string(),
            CompileError::UnexpectedReturnType => "expected `ContractResult`".to_string(),
            CompileError::ExpectedIdentifier => "expected identifier".to_string(),
            CompileError::ExpectedFunctionArgs => "expected function args".to_string(),
            CompileError::DuplicateIdentifier => {
                "identifier used as parameter more than once".to_string()
            }
        }
    }
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn attr_with_args(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut f: syn::ItemFn = parse_macro_input!(input);

    let invoke_fn_name = f.sig.ident.to_string();
    let export_fn_name = format!("x{}", hex::encode(invoke_fn_name.as_bytes()));

    // replace `export_fn_name` with `invoke_fn_name`
    f.sig.ident = syn::Ident::new(&invoke_fn_name, f.sig.ident.span());

    // parse function params
    let (fn_args, fn_return_type) = parse_fn_params(&f);

    unsafe {
        CONTRACT_DOC.push(ContractFnSig {
            fn_name: export_fn_name.clone(),
            input_type: fn_args.iter().map(|arg| arg.r#type.ty.clone()).collect(),
            output_type: if let Some(param_type) = &fn_return_type {
                Some(param_type.ty.clone())
            } else {
                None
            },
        });
        // println!("{:?}", CONTRACT_DOC);
    }

    let mut token = generate_invoke_fn(export_fn_name, invoke_fn_name, fn_args, fn_return_type);

    token.append_all(quote! { #f });

    token.into()
}

fn generate_invoke_fn(
    export_fn_name: String,
    user_fn_name: String,
    fn_args: Vec<FnArg>,
    fn_return_type: Option<ParamType>,
) -> TokenStream2 {
    let (mut parse_input_body, mut call_method_param_body) = (
        quote! {
            extern crate c123chain_cdk;

            use c123chain_cdk::codec::Source;
            use c123chain_cdk::runtime;

            let deps = runtime::make_dependencies();
            let input = deps.api.input();
            let input = Source::new(&input);

        },
        quote! {},
    );

    for (i, fn_arg) in fn_args.iter().enumerate() {
        let ident_name = &fn_arg.ident;
        // let ident_type = &fn_arg.r#type.token;

        parse_input_body.append_all(quote! {
            let #ident_name = input.read().unwrap();
        });

        if i == 0 {
            call_method_param_body.append_all(quote! {
                #ident_name
            });
        } else {
            call_method_param_body.append_all(quote! {
                , #ident_name
            });
        }
    }
    let user_fn_idet = format_ident!("{}", user_fn_name);
    if let Some(_) = fn_return_type {
        parse_input_body.append_all(quote! {
            let _contract_result = #user_fn_idet(#call_method_param_body);
            deps.api.ret(_contract_result);
        });
    } else {
        parse_input_body.append_all(quote! {
            #user_fn_idet(#call_method_param_body);
        });
    }

    let tokens = quote! {
        #[no_mangle]
        fn invoke() {
            #parse_input_body
        }
    };
    let mut f: syn::ItemFn = syn::parse2(tokens).unwrap();
    f.sig.ident = syn::Ident::new(&export_fn_name, f.sig.ident.span());
    f.to_token_stream()
}

#[derive(Debug)]
struct FnArg {
    ident: syn::Ident,
    r#type: ParamType,
}

#[derive(Debug, Clone)]
struct ParamType {
    ty: String,
    token: TokenStream2,
}

fn parse_fn_params(f: &syn::ItemFn) -> (Vec<FnArg>, Option<ParamType>) {
    let mut fn_input_params = vec![];

    let mut name_set = HashSet::new();

    for fn_arg in f.sig.inputs.iter() {
        match fn_arg {
            syn::FnArg::Typed(typed) => match &*typed.pat {
                syn::Pat::Ident(ident) => {
                    let name = ident.ident.to_string();

                    if name_set.contains(name.as_str()) {
                        abort!(ident, CompileError::DuplicateIdentifier);
                    }
                    name_set.insert(name);

                    fn_input_params.push(FnArg {
                        ident: ident.ident.clone(),
                        r#type: parse_arg_type(&*typed.ty),
                    });
                }
                _ => {
                    abort!(typed, CompileError::ExpectedIdentifier);
                }
            },
            syn::FnArg::Receiver(_) => {
                abort!(fn_arg, CompileError::ExpectedFunctionArgs);
            }
        }
    }

    let fn_output_param_type = match &f.sig.output {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, tp) => Some(parse_return_type(&*tp)),
    };

    // (input args, return arg)
    (fn_input_params, fn_output_param_type)
}

fn parse_arg_type(arg_tp: &syn::Type) -> ParamType {
    macro_rules! gt1 {
        ($i: ident) => {
            ParamType {
                ty: String::from(stringify!($i)),
                token: quote! { $i },
            }
        };
    }

    macro_rules! gt2 {
        ($i: ident, $token: ty) => {
            ParamType {
                ty: String::from(stringify!($i)),
                token: quote! { $token },
            }
        };
    }

    let supported_types = vec![
        gt1!(bool),
        gt1!(u32),
        gt1!(i32),
        gt1!(u64),
        gt1!(i64),
        gt1!(u128),
        gt1!(i128),
        gt1!(String),
    ];
    let st_str = gt2!(str, &str);
    let st_slice_u8 = gt2!(u8, &[u8]);

    match arg_tp {
        syn::Type::Path(tp) => {
            for t in supported_types.iter() {
                if tp.path.is_ident(&t.ty) {
                    return t.clone();
                }
            }
            abort!(tp, CompileError::UnexpectedArgType);
        }
        syn::Type::Reference(tr) => match &*tr.elem {
            syn::Type::Path(tp) => {
                if tp.path.is_ident(&st_str.ty) {
                    return st_str;
                } else {
                    abort!(tp, CompileError::UnexpectedArgType);
                }
            }
            syn::Type::Slice(ts) => match &*ts.elem {
                syn::Type::Path(tp) => {
                    if tp.path.is_ident(&st_slice_u8.ty) {
                        return st_slice_u8;
                    } else {
                        abort!(tp, CompileError::UnexpectedArgType);
                    }
                }
                _ => {
                    abort!(ts, CompileError::UnexpectedArgType);
                }
            },
            _ => {
                abort!(tr, CompileError::UnexpectedArgType);
            }
        },
        _ => abort!(arg_tp, CompileError::UnexpectedArgType),
    }
}

fn parse_return_type(return_tp: &syn::Type) -> ParamType {
    let param_type_result = ParamType {
        ty: "ContractResult".to_string(),
        token: quote! {},
    };

    match return_tp {
        syn::Type::Path(tp) => {
            if tp.path.is_ident(&param_type_result.ty) {
                return param_type_result;
            } else {
                abort!(tp, CompileError::UnexpectedReturnType);
            }
        }
        _ => {
            abort!(return_tp, CompileError::UnexpectedReturnType);
        }
    };
}
