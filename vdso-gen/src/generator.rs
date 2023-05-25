use std::collections::HashMap;

use super::Ver;
use proc_macro2::{Delimiter, Group, Literal, Punct, Spacing, Span, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

pub(crate) fn generate_declarations(
    m_vs: &Vec<Box<str>>,
    o_vs: &Vec<Box<str>>,
    m_fs: &Vec<(Box<str>, Box<str>, Ver)>,
    o_fs: &Vec<(Box<str>, Box<str>, Ver)>,
    tokens: &mut TokenStream,
) {
    for i in 0..m_vs.len() {
        DefineVer(Ver::Mandatory(i)).to_tokens(tokens);
    }
    for i in 0..o_vs.len() {
        DefineVer(Ver::Optional(i)).to_tokens(tokens);
    }
    let null = quote!(::core::ptr::null());
    tokens.append(ident("let"));
    tokens.append(ident("mut"));
    tokens.append(ident("vdso_inst"));
    tokens.append(Punct::new('=', Spacing::Alone));
    tokens.append(ident("Self"));
    tokens.append(Group::new(Delimiter::Brace, {
        let mut res = TokenStream::new();
        let tokens = &mut res;
        for (v, _, _) in m_fs {
            tokens.append(format_ident!("{}", v.as_ref()));
            tokens.append(Punct::new(':', Spacing::Alone));
            null.to_tokens(tokens);
            tokens.append(Punct::new(',', Spacing::Alone));
        }
        for (v, _, _) in o_fs {
            tokens.append(format_ident!("{}", v.as_ref()));
            tokens.append(Punct::new(':', Spacing::Alone));
            null.to_tokens(tokens);
            tokens.append(Punct::new(',', Spacing::Alone));
        }
        res
    }));
    tokens.append(Punct::new(';', Spacing::Alone));
}

struct CStr<'a>(&'a str);
impl<'a> ToTokens for CStr<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Group::new(Delimiter::Bracket, {
            let mut tokens = TokenStream::new();
            for n in self.0.as_bytes().iter().copied() {
                tokens.append(Literal::u8_unsuffixed(n));
                tokens.append(Punct::new(',', Spacing::Alone));
            }
            tokens.append(Literal::u8_suffixed(0));
            tokens
        }));
        tokens.append(Group::new(Delimiter::Bracket, quote!(..)));
        tokens.append(Punct::new('.', Spacing::Alone));
        tokens.append(ident("as_ptr"));
        tokens.append(Group::new(Delimiter::Parenthesis, TokenStream::new()));
    }
}

pub(crate) fn generate_versions_parser(m_vs: &Vec<Box<str>>, o_vs: &Vec<Box<str>>) -> TokenStream {
    fn generate_branch_m(name: &str, n: usize) -> TokenStream {
        let raw_name = CStr(name);

        let var = Ver::Mandatory(n);
        quote! {
            if crate::util::streq(version.name(), #raw_name) {
                if #var == 0 {
                    mandatory_count += 1;
                    #var = version.id();
                } else {
                    return ::core::option::Option::None;
                }
            }
        }
    }

    fn generate_branch_o(name: &str, n: usize) -> TokenStream {
        let raw_name = CStr(name);

        let var = Ver::Optional(n);
        quote! {
            if crate::util::streq(version.name(), #raw_name) {
                if #var != 0 {
                    return ::core::option::Option::None;
                }
                #var = version.id();
            }
        }
    }

    #[allow(clippy::ptr_arg)]
    fn generate_match(m_vs: &Vec<Box<str>>, o_vs: &Vec<Box<str>>) -> TokenStream {
        let mut branches = HashMap::<u32, TokenStream>::new();

        for (i, ver) in m_vs.iter().enumerate() {
            let ver = ver.as_ref();
            branches
                .entry(elf_hash(ver.as_bytes()))
                .or_default()
                .extend(generate_branch_m(ver, i));
        }
        for (i, ver) in o_vs.iter().enumerate() {
            let ver = ver.as_ref();
            branches
                .entry(elf_hash(ver.as_bytes()))
                .or_default()
                .extend(generate_branch_o(ver, i));
        }
        let mut branches = branches.into_iter().collect::<Vec<(_, _)>>();
        branches.sort_by(|a, b| a.0.cmp(&b.0));

        let mut tokens = TokenStream::new();
        for (hash, branches) in branches {
            tokens.append(Literal::u32_unsuffixed(hash));
            FatArrow.to_tokens(&mut tokens);
            tokens.append(Group::new(Delimiter::Brace, branches));
        }
        tokens
    }

    let mut tokens = {
        let forbody = {
            let mut tokens = quote!(match version.hash());
            tokens.append(Group::new(Delimiter::Brace, {
                let mut m = generate_match(m_vs, o_vs);
                EmptyBranch.to_tokens(&mut m);
                m
            }));
            if o_vs.is_empty() {
                tokens.append(ident("if"));
                tokens.append(ident("mandatory_count"));
                tokens.append(Punct::new('=', Spacing::Joint));
                tokens.append(Punct::new('=', Spacing::Alone));
                tokens.append(Literal::usize_unsuffixed(m_vs.len()));
                tokens.append(Group::new(Delimiter::Brace, quote!(break;)));
            }
            tokens
        };

        let mut tokens = quote! {
            let mut mandatory_count = 0usize;
            for version in reader.versions()
        };
        tokens.append(Group::new(Delimiter::Brace, forbody));
        tokens
    };

    MandatoryCheck(m_vs.len()).to_tokens(&mut tokens);

    tokens
}

fn generate_symbols_parser(
    m_fs: &Vec<(Box<str>, Box<str>, Ver)>,
    o_fs: &Vec<(Box<str>, Box<str>, Ver)>,
) -> TokenStream {
    fn generate_branch_m(id: &str, name: &str, ver: Ver) -> TokenStream {
        let raw_name = CStr(name);
        let ident = ident(id);

        let mut cond = if matches!(ver, Ver::None) {
            quote!(if crate::util::streq(symbol.name(), #raw_name))
        } else {
            quote!(if crate::util::streq(symbol.name(), #raw_name) && Some(#ver) == symbol.version_id())
        };

        cond.append(Group::new(
            Delimiter::Brace,
            quote! {
                if vdso_inst.#ident.is_null() {
                    mandatory_count += 1;
                    vdso_inst.#ident = symbol.ptr();
                } else {
                    return ::core::option::Option::None;
                }
            },
        ));

        cond
    }

    fn generate_branch_o(id: &str, name: &str, ver: Ver) -> TokenStream {
        let raw_name = CStr(name);
        let ident = ident(id);

        let mut cond = if matches!(ver, Ver::None) {
            quote!(if crate::util::streq(symbol.name(), #raw_name))
        } else {
            quote!(if crate::util::streq(symbol.name(), #raw_name) && Some(#ver) == symbol.version_id())
        };

        cond.append(Group::new(
            Delimiter::Brace,
            quote! {
                if !vdso_inst.#ident.is_null() {
                    return ::core::option::Option::None;
                }
                vdso_inst.#ident = symbol.ptr();
            },
        ));

        cond
    }

    #[allow(clippy::ptr_arg)]
    fn generate_match(
        m_fs: &Vec<(Box<str>, Box<str>, Ver)>,
        o_fs: &Vec<(Box<str>, Box<str>, Ver)>,
    ) -> TokenStream {
        let mut branches = HashMap::<u32, TokenStream>::new();

        for (id, name, ver) in m_fs.iter() {
            let id = id.as_ref();
            let name = name.as_ref();
            let ver = *ver;

            branches
                .entry(elf_hash(name.as_bytes()))
                .or_default()
                .extend(generate_branch_m(id, name, ver));
        }
        for (id, name, ver) in o_fs.iter() {
            let id = id.as_ref();
            let name = name.as_ref();
            let ver = *ver;

            branches
                .entry(elf_hash(name.as_bytes()))
                .or_default()
                .extend(generate_branch_o(id, name, ver));
        }

        let mut branches = branches.into_iter().collect::<Vec<(_, _)>>();
        branches.sort_by(|a, b| a.0.cmp(&b.0));

        let mut tokens = TokenStream::new();
        for (hash, branches) in branches {
            tokens.append(Literal::u32_unsuffixed(hash));
            tokens.append(Punct::new('=', Spacing::Joint));
            tokens.append(Punct::new('>', Spacing::Alone));
            tokens.append(Group::new(Delimiter::Brace, branches));
        }
        tokens
    }

    let mut tokens = {
        let forbody = {
            let mut tokens = quote!(match crate::util::elf_hash(symbol.name()));
            tokens.append(Group::new(Delimiter::Brace, {
                let mut m = generate_match(m_fs, o_fs);
                EmptyBranch.to_tokens(&mut m);
                m
            }));
            if o_fs.is_empty() {
                tokens.append(ident("if"));
                tokens.append(ident("mandatory_count"));
                tokens.append(Punct::new('=', Spacing::Joint));
                tokens.append(Punct::new('=', Spacing::Alone));
                tokens.append(Literal::usize_unsuffixed(m_fs.len()));
                tokens.append(Group::new(Delimiter::Brace, quote!(break;)));
            }
            tokens
        };

        let mut tokens = quote! {
            let mut mandatory_count = 0usize;
            for symbol in reader.symbols()
        };
        tokens.append(Group::new(Delimiter::Brace, forbody));
        tokens
    };

    MandatoryCheck(m_fs.len()).to_tokens(&mut tokens);

    tokens
}

pub(crate) fn generate(
    m_vs: &Vec<Box<str>>,
    o_vs: &Vec<Box<str>>,
    m_fs: &Vec<(Box<str>, Box<str>, Ver)>,
    o_fs: &Vec<(Box<str>, Box<str>, Ver)>,
) -> TokenStream {
    let mut res = TokenStream::new();
    let tokens = &mut res;
    generate_declarations(m_vs, o_vs, m_fs, o_fs, tokens);
    tokens.append(Group::new(
        Delimiter::Brace,
        generate_versions_parser(m_vs, o_vs),
    ));
    tokens.append(Group::new(
        Delimiter::Brace,
        generate_symbols_parser(m_fs, o_fs),
    ));
    tokens.append(ident("Some"));
    tokens.append(Group::new(Delimiter::Parenthesis, quote!(vdso_inst)));
    res
}

fn elf_hash(name: &[u8]) -> u32 {
    let mut h: u32 = 0;
    unsafe {
        let mut ptr = name.as_ptr();
        let endptr = ptr.add(name.len());

        #[allow(clippy::transmutes_expressible_as_ptr_casts)]
        while core::mem::transmute::<_, usize>(ptr) < core::mem::transmute::<_, usize>(endptr) {
            h = (h << 4).wrapping_add((*ptr) as u32);
            let g = h & 0xf000_0000;
            if g != 0 {
                h ^= g >> 24;
            }
            h &= !g;

            ptr = ptr.add(1);
        }
    }
    h
}

struct MandatoryCheck(usize);

impl ToTokens for MandatoryCheck {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(ident("if"));
        tokens.append(ident("mandatory_count"));
        NE.to_tokens(tokens);
        tokens.append(Literal::usize_unsuffixed(self.0));
        tokens.append(Group::new(
            Delimiter::Brace,
            quote!(return ::core::option::Option::None;),
        ));
    }
}

struct NE;

impl ToTokens for NE {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Punct::new('!', Spacing::Joint));
        tokens.append(Punct::new('=', Spacing::Alone));
    }
}

fn ident<S: AsRef<str>>(name: S) -> proc_macro2::Ident {
    proc_macro2::Ident::new(name.as_ref(), Span::call_site())
}

struct DefineVer(Ver);

impl ToTokens for DefineVer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(ident("let"));
        tokens.append(ident("mut"));
        self.0.to_tokens(tokens);
        tokens.append(Punct::new('=', Spacing::Alone));
        tokens.append(Literal::u16_suffixed(0));
        tokens.append(Punct::new(';', Spacing::Alone));
    }
}

impl ToTokens for Ver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(match self {
            Self::Mandatory(n) => format_ident!("version_mandatory_{}", n),
            Self::Optional(n) => format_ident!("version_optional_{}", n),
            _ => unreachable!(),
        });
    }
}

struct FatArrow;
impl ToTokens for FatArrow {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Punct::new('=', Spacing::Joint));
        tokens.append(Punct::new('>', Spacing::Joint));
    }
}

struct EmptyBranch;
impl ToTokens for EmptyBranch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(ident("_"));
        FatArrow.to_tokens(tokens);
        tokens.append(Group::new(Delimiter::Parenthesis, TokenStream::new()));
        tokens.append(Punct::new(',', Spacing::Alone));
    }
}
