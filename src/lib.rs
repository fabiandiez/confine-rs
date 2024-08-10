#![allow(unused)]

mod config;

use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use std::io::ErrorKind;
use syn::ItemStruct;

#[derive(thiserror::Error, Debug)]
enum ConfineError {
    #[error("Error parsing attribute arguments: {0}")]
    ParseError(#[from] Error),
    
    #[error("Error parsing macro input: {0}")]
    ParseMacroInput(#[from] syn::Error),

    #[error("The variable to configure the environment ('{0}') was not set.")]
    EnvVarNotSet(String),
    
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
pub fn confine_secret(args: TokenStream, input: TokenStream) -> TokenStream {
    input
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

    let struct_name = &input.ident;

    let mut fields = Vec::new();
    for field in input.fields.iter() {
        let field_name = field.ident.as_ref().expect("Field must have a name.");
        fields.push(quote::quote!(
            #field_name: 42,
        ));
    }

    Ok(
        quote::quote!(
        #input
    )
            .into())
}

fn get_env(args: &ConfineArgs) -> Result<Option<String>, ConfineError> {
    let current_env = match std::env::var(args.env_var()) {
        Ok(v) => Some(v),
        Err(_) => {
            #[cfg(not(debug_assertions))]
            return Err(EnvVarNotSet(args.env_var()).into());

            #[cfg(debug_assertions)]
            None
        }
    };
    Ok(current_env)
}
