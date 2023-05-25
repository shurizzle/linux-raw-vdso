use quote::{quote, ToTokens};
use syn::{braced, parse::Parse, Token};

pub struct VdsoStruct {
    pub attributes: Vec<syn::Attribute>,
    pub visibility: syn::Visibility,
    pub struct_token: Token![struct],
    pub name: syn::Ident,
    pub brace_token: syn::token::Brace,
    pub fields: syn::punctuated::Punctuated<VdsoField, Token![,]>,
}

pub struct VdsoField {
    pub attributes: Vec<syn::Attribute>,
    pub visibility: syn::Visibility,
    pub ident: syn::Ident,
    pub optional: Option<Token![?]>,
    pub colon: Token![:],
    pub name: syn::Ident,
    pub version: Option<VdsoFieldVersion>,
}

pub struct VdsoFieldVersion {
    pub at: Token![@],
    pub version: syn::LitStr,
}

impl Parse for VdsoFieldVersion {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            at: input.parse()?,
            version: input.parse()?,
        })
    }
}

impl Parse for VdsoField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attributes = input.call(syn::Attribute::parse_outer)?;
        let visibility = input.parse()?;
        let ident = input.parse()?;
        let optional = if input.peek(Token![?]) {
            Some(input.parse()?)
        } else {
            None
        };
        let colon = input.parse()?;
        let name = input.parse()?;
        let version = if input.peek(Token![@]) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self {
            attributes,
            visibility,
            ident,
            optional,
            colon,
            name,
            version,
        })
    }
}

impl Parse for VdsoStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attributes = input.call(syn::Attribute::parse_outer)?;
        let visibility: syn::Visibility = input.parse()?;
        let struct_token = input.parse::<Token![struct]>()?;
        let name: syn::Ident = input.parse()?;
        let content;
        let brace_token = braced!(content in input);
        let fields = content.parse_terminated(VdsoField::parse, Token![,])?;

        Ok(Self {
            attributes,
            visibility,
            struct_token,
            name,
            brace_token,
            fields,
        })
    }
}

impl ToTokens for VdsoField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for a in &self.attributes {
            a.to_tokens(tokens);
        }
        self.visibility.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.colon.to_tokens(tokens);
        quote!(*const ::core::ffi::c_void).to_tokens(tokens);
    }
}

impl ToTokens for VdsoStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for a in &self.attributes {
            a.to_tokens(tokens);
        }
        self.visibility.to_tokens(tokens);
        self.struct_token.to_tokens(tokens);
        self.name.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            self.fields.to_tokens(tokens);
        });
    }
}
