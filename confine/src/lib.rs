#![allow(unused)]

mod config;

use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use std::io::ErrorKind;
use quote::quote;
use syn::ItemStruct;
use toml::Value;
use crate::config::ConfigValue;
use crate::config::ConfigError;

#[derive(thiserror::Error, Debug)]
enum ConfineError {
    #[error("Error parsing attribute arguments: {0}")]
    ParseError(#[from] Error),

    #[error("Error parsing macro input: {0}")]
    ParseMacroInput(#[from] syn::Error),

    #[error("The variable to configure the environment ('{0}') was not set.")]
    EnvVarNotSet(String),

    #[error("Unsupported type: {0}. Currently only supports String, i64, f64, and bool.")]
    UnsupportedType(String),

    #[error("Config file error: {0}")]
    ConfigError(#[from] config::ConfigError),
}

impl From<ConfineError> for TokenStream {
    fn from(value: ConfineError) -> Self {
        let error = Error::custom(value);
        TokenStream::from(error.write_errors())
    }
}

#[derive(Debug, FromMeta)]
struct ConfineArgs {
    env_var: Option<String>,
    path: Option<String>,
    prefix: Option<String>,
}

impl ConfineArgs {
    fn env_var(&self) -> String {
        self.env_var.clone().unwrap_or("CONFINE_ENV".to_string())
    }

    fn path(&self) -> String {
        self.path.clone().unwrap_or("config".to_string())
    }

    fn prefix(&self) -> String {
        self.prefix.clone().unwrap_or("application".to_string())
    }
}

#[proc_macro_attribute]
pub fn confine(args: TokenStream, input: TokenStream) -> TokenStream {
    process_confine(args, input).unwrap_or_else(|e| e.into())
}

fn process_confine(args: TokenStream, input: TokenStream) -> Result<TokenStream, ConfineError> {
    let attr_args = NestedMeta::parse_meta_list(args.into())?;
    let input = syn::parse::<ItemStruct>(input)?;
    let args = ConfineArgs::from_list(&attr_args)?;
    let current_env = get_env(&args)?;
    let config = config::Config::new(args.prefix(), args.path(), current_env)?;


    let mut fields = Vec::new();
    let mut loaded_fields = Vec::new();
    for field in input.fields.iter() {
        let field_name = field.ident.as_ref().expect("Field must have a name.");
        let field_type = &field.ty;
        fields.push(quote!(
            #field_name: #field_type,
        ));

        let field_type_str = quote!(#field_type).to_string();
        if !["String", "i64", "f64", "bool"].contains(&field_type_str.as_str()) {
            return Err(ConfineError::UnsupportedType(field_type_str));
        }

        let config_value = match config.get(&field_name.to_string(), &field_type_str)? {
            ConfigValue::String(v) => quote! { #v.to_string() },
            ConfigValue::Int(v) => quote! { #v },
            ConfigValue::Float(v) => quote! { #v },
            ConfigValue::Bool(v) => quote! { #v },
        };
       
        loaded_fields.push(quote!(
            #field_name: #config_value,
        ));
    }

    let struct_name = &input.ident;
    let vis = &input.vis;

    let out_struct = quote!(
        #vis struct #struct_name {
            #(#fields)*
        });

    let out_impl = quote!(
        impl #struct_name {
            pub fn load() -> Self {
                Self {
                    #(#loaded_fields)*
                }
            }
        }
    );

    let out = quote!(
        #out_struct
        #out_impl
    );

    dbg!(out.to_string());

    Ok(out.into())
}

fn get_env(args: &ConfineArgs) -> Result<Option<String>, ConfineError> {
    let current_env = match std::env::var(args.env_var()) {
        Ok(v) => Some(v),
        Err(_) => {
            #[cfg(not(debug_assertions))]
            return Err(ConfineError::EnvVarNotSet(args.env_var()).into());

            #[cfg(debug_assertions)]
            None
        }
    };
    Ok(current_env)
}