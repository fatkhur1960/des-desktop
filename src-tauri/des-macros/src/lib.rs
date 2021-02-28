#![recursion_limit = "128"]
#![allow(unused_imports, unused_assignments, unused_mut, dead_code)]

extern crate heck;
extern crate lazy_static;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate syn;

use proc_macro2::{Delimiter, Group, Ident, Literal, Span, TokenStream, TokenTree};
use quote::quote;
use serde::{Deserialize, Serialize};
// use syn;

use std::iter::FromIterator;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct ApiEndpoint {
    pub group: String,
    pub param: String,
    pub method_name: String,
}

#[proc_macro_attribute]
pub fn service(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut _void = String::new();
    let mut struct_name = String::new();
    let mut to_update = &mut _void;

    // let attr_args = parse_macro_input!(_attr as syn::AttributeArgs);
    let items = proc_macro2::TokenStream::from(item.clone());
    for item in items.clone() {
        match &item {
            TokenTree::Ident(ident) if ident.to_string() == "impl" => {
                to_update = &mut struct_name;
            }
            TokenTree::Ident(ident) => {
                *to_update = ident.to_string();
                to_update = &mut _void;
            }
            _ => (),
        }
    }

    let mut api_endpoint_info: Vec<ApiEndpoint> = vec![];

    let mut new_items: proc_macro2::TokenStream = {
        let mut to_update = &mut _void;
        let items = proc_macro2::TokenStream::from(item.clone());

        for item in items.clone() {
            match &item {
                TokenTree::Ident(ident) if ident.to_string() == "impl" => {
                    to_update = &mut struct_name;
                }
                TokenTree::Ident(ident) => {
                    *to_update = ident.to_string();
                    to_update = &mut _void;
                }
                _ => (),
            }
        }

        let new_items: proc_macro2::TokenStream = items
            .into_iter()
            .map(|item| {
                match &item {
                    TokenTree::Group(group) => {
                        let mut tb: Vec<TokenTree> = vec![];
                        let items = group.stream().into_iter();

                        TokenTree::Group(Group::new(
                            group.delimiter(),
                            items
                                .map(|item| {
                                    match &item {
                                        TokenTree::Group(group) => {
                                            let items = group.stream().into_iter();
                                            let mut is_api_endpoint = false;
                                            let mut after_payload = false;

                                            tb.push(item.clone());

                                            TokenTree::Group(Group::new(
                                                group.delimiter(),
                                                items
                                                    .map(|item| {
                                                        let mut new_tt: proc_macro2::TokenTree =
                                                            item.clone();
                                                        match &item {
                                                            TokenTree::Ident(ident) => {
                                                                if after_payload {
                                                                    api_endpoint_info
                                                                        .last_mut()
                                                                        .map(|info| {
                                                                            info.param =
                                                                                ident.to_string();
                                                                        });
                                                                }
                                                                match ident.to_string().as_ref() {
                                                                    "route" => {
                                                                        after_payload = false;
                                                                        is_api_endpoint = true;
                                                                        new_tt = item.clone();

                                                                        let info =
                                                                            gather_endpoint_info(
                                                                                group.stream(),
                                                                                &struct_name,
                                                                            );

                                                                        api_endpoint_info
                                                                            .push(info);
                                                                    }
                                                                    "payload" => {
                                                                        after_payload = true;
                                                                    }
                                                                    _ => (),
                                                                }
                                                            }

                                                            _ => (),
                                                        }

                                                        if is_api_endpoint {
                                                            // dbg!(&new_tt);
                                                            new_tt
                                                        } else {
                                                            item
                                                        }
                                                    })
                                                    .collect(),
                                            ))
                                        }
                                        TokenTree::Ident(ident) => {
                                            if api_endpoint_info
                                                .last()
                                                .map(|a| a.method_name.is_empty())
                                                == Some(true)
                                                && !tb.is_empty()
                                                && tb[tb.len() - 1].to_string() == "fn"
                                            {
                                                api_endpoint_info.last_mut().map(|info| {
                                                    info.method_name = ident.to_string();
                                                });
                                            }

                                            tb.push(item.clone());

                                            item
                                        }
                                        _ => {
                                            tb.push(item.clone());
                                            item
                                        }
                                    }
                                })
                                .collect(),
                        ))
                    }
                    _ => item,
                }
            })
            .collect();
        new_items
    };

    // dbg!(&api_endpoint_info);

    // buatkan auto wire interface method
    let tts = {
        let struct_name = Ident::new(&struct_name, Span::call_site());
        let mut sas = vec![];
        for aei in &api_endpoint_info {
            let ty = Ident::new(&aei.param, Span::call_site());
            let method_name = Ident::new(&aei.method_name, Span::call_site());
            let method = aei.method_name.clone();
            sas.push(quote! {
                debug!("Calling Service {}::{} | payload: {:?}", stringify!(#struct_name), #method, &payload);
                if target == #method {
                    return match serde_json::from_value::<#ty>(payload) {
                        Ok(payload) => {
                            #struct_name::#method_name(&state, payload)
                                .map(|res| serde_json::to_value(res).unwrap())
                                .map_err(|e| crate::ServiceError::new(e.to_string()))
                        },
                        Err(e) => Err(crate::ServiceError::new(e.to_string()))
                    };
                }
            });
        }
        let sases = TokenStream::from_iter(sas.into_iter());
        quote! {
            impl #struct_name {
                #[doc(hidden)]
                pub fn wire(target: std::string::String, payload: serde_json::Value) -> Result<serde_json::Value, crate::ServiceError> {
                    let state = crate::service::AppState::new();
                    #sases

                    Err(crate::ServiceError::new("Unknown route".to_string()))
                }
            }
        }
    };

    new_items.extend(tts);
    proc_macro::TokenStream::from(new_items)
}

#[proc_macro_attribute]
pub fn route(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut func_name = "".to_string();
    let debug = false;

    let item2 = proc_macro2::TokenStream::from(item);

    let items = item2.into_iter();

    #[allow(unused_assignments)]
    let mut no_add = false;

    let mut in_fn = 0;
    let mut after_fn = false;
    let mut group_cnt = 0;
    let mut in_open_fn = false;
    let mut return_wrapped = false;
    let mut tb: Vec<TokenTree> = vec![];
    let mut begin_capture_result_type = false;
    let mut result_type: Vec<TokenTree> = vec![];

    for item in items {
        no_add = false;

        if begin_capture_result_type {
            match &item {
                TokenTree::Group(ref group) => {
                    let end_capture = group.delimiter() == Delimiter::Brace;
                    begin_capture_result_type = !end_capture;
                    if end_capture {
                        let rettype = TokenStream::from_iter(result_type.clone().into_iter());
                        let new_return_type = quote! {
                            crate::service::Result<#rettype>
                        };
                        for r in new_return_type {
                            tb.push(r);
                        }
                        return_wrapped = true;
                    }
                }
                _ => {
                    result_type.push(item.clone());
                    continue;
                }
            }
        }

        if item.to_string() == "fn" {
            in_fn = 1;
            tb.push(item);
            continue;
        }

        if in_fn == 1 && !after_fn {
            after_fn = true;
            func_name = item.to_string();
            tb.push(item);
            continue;
        }

        // dbg!((group_cnt, after_fn, in_fn, has_http_req));

        if after_fn {
            let mut query_type: Vec<TokenTree> = vec![];
            match item {
                TokenTree::Group(ref group) => {
                    group_cnt += 1;
                    in_open_fn = group.delimiter() == Delimiter::Brace;

                    if group_cnt == 1 {
                        if let TokenTree::Group(ref group) = item {
                            let mut in_query = false;
                            let mut begin_capture_query_type = false;
                            for inner in group.stream() {
                                match inner {
                                    TokenTree::Ident(ref ident) => {
                                        if ident.to_string() == "payload" {
                                            in_query = true;
                                        } else if in_query {
                                            in_query = false;
                                            begin_capture_query_type = true;
                                            query_type.push(inner.clone());
                                        } else if begin_capture_query_type {
                                            query_type.push(inner.clone());
                                        }
                                    }
                                    TokenTree::Group(ref g) => {
                                        if in_query
                                            && g.delimiter() == Delimiter::Parenthesis
                                            && !begin_capture_query_type
                                        {
                                            in_query = false;
                                            query_type.push(inner.clone());
                                        } else if begin_capture_query_type {
                                            query_type.push(inner.clone());
                                        }
                                    }
                                    TokenTree::Punct(ref punct) => {
                                        if begin_capture_query_type {
                                            if punct.to_string() == "," {
                                                begin_capture_query_type = false;
                                            } else {
                                                query_type.push(inner.clone());
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }

                        if query_type.is_empty() {
                            panic!(
                                "Fungsi `{}` perlu ada `payload` parameter-nya, \
                                 contoh: `pub {}(payload: Payload) -> JsonValue`.",
                                func_name, func_name
                            );
                        }

                        let query_type = TokenStream::from_iter(query_type.into_iter());

                        if debug {
                            // dbg!(&query_type);
                        }

                        let group = Group::new(
                            Delimiter::Parenthesis,
                            TokenStream::from_iter(
                                quote! {
                                    state: &crate::service::AppState, payload: #query_type
                                }
                                .into_iter(),
                            ),
                        );
                        let tt: TokenTree = TokenTree::Group(group);
                        tb.push(tt);
                        // prev_token = item.clone();
                        continue;
                    }
                }
                _ => (),
            }
            if group_cnt >= 1 && !return_wrapped {
                // wrap return value menggunakan ApiResult<>
                match (&tb.get(tb.len() - 2), &tb.last()) {
                    (Some(&TokenTree::Punct(ref punct1)), Some(&TokenTree::Punct(ref punct2))) => {
                        if punct1.as_char() == '-' && punct2.as_char() == '>' {
                            begin_capture_result_type = true;
                            result_type.push(item.clone());
                            continue;
                        }
                    }
                    _ => (),
                }
            }

            if group_cnt > 1 && in_fn < 2 && in_open_fn {
                in_fn = 2;

                if let TokenTree::Group(ref group) = item {
                    let mut new_stream = vec![];

                    new_stream.push(group.stream());

                    let group = Group::new(
                        Delimiter::Brace,
                        TokenStream::from_iter(new_stream.into_iter()),
                    );
                    let tt: TokenTree = TokenTree::Group(group);
                    tb.push(tt);
                }
                continue;
            }
        }

        if !no_add {
            tb.push(item);
        }
    }

    if debug {
        // dbg!(&tb);
    }

    proc_macro::TokenStream::from(TokenStream::from_iter(tb.into_iter()))
}

// fn get_lit_str(lit: &proc_macro2::Literal) -> String {
//     let a = lit.to_string();
//     a[1..a.len() - 1].trim().to_string()
// }

fn gather_endpoint_info(_stream: TokenStream, base: &str) -> ApiEndpoint {
    ApiEndpoint {
        group: base.to_string(),
        method_name: Default::default(),
        ..Default::default()
    }
}
