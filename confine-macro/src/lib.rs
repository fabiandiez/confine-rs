mod config;

use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemStruct;

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
    ConfigBuilderError(#[from] confine_builder::ConfineBuilderError),
}

impl From<ConfineError> for TokenStream {
    fn from(value: ConfineError) -> Self {
        let error = Error::custom(value);
        TokenStream::from(error.write_errors())
    }
}

#[derive(Debug, FromMeta)]
struct ConfineArgs {
    pub default_env: Option<String>,
    env_var: Option<String>,
    path: Option<String>,
    prefix: Option<String>,
}

impl ConfineArgs {
    fn env_var(&self) -> String {
        self.env_var.clone().unwrap_or("CONFINE_ENV".to_string())
    }

    fn path(&self) -> String {
        self.path.clone().unwrap_or("config".into())
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
    let args = ConfineArgs::from_list(&attr_args)?;
    let input = syn::parse::<ItemStruct>(input)?;
    let struct_name = &input.ident;

    let path = args.path();
    let env_var = args.env_var();
    let prefix = args.prefix();

    Ok(quote!(
        #input

        impl #struct_name {
            fn try_load() -> Result<Self, confine::ConfineBuilderError> {
                let config = confine::ConfineConfigBuilder::default()
                    .config_path(#path.into())
                    .env_var(#env_var.into())
                    .prefix(#prefix.into())
                    .try_load::<#struct_name>()?;
                Ok(config)
            }
        }
    )
    .into())
}
