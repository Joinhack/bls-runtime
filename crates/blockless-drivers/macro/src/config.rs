use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Result, Token};
use wiggle_generate::config::{Paths, WitxConf};

pub struct BlocklessConfig {
    pub c: WitxConf,
    pub link_method: syn::LitStr,
    pub target: syn::Path,
}

enum BlocklessConfigField {
    WitxField(Paths),
    LinkMethodField(syn::LitStr),
    TargetField(syn::Path),
}

mod kw {
    syn::custom_keyword!(witx);
    syn::custom_keyword!(target);
    syn::custom_keyword!(link_method);
}

/// The Blockless Configure for the Witx File, use Witx genrate the code of linker abi.
impl BlocklessConfig {
    fn build(fields: impl Iterator<Item = BlocklessConfigField>) -> Result<Self> {
        let mut witx_confg = None;
        let mut target = None;
        let mut link_method = None;
        for f in fields {
            match f {
                BlocklessConfigField::TargetField(t) => target = Some(t),
                BlocklessConfigField::LinkMethodField(m) => link_method = Some(m),
                BlocklessConfigField::WitxField(paths) => {
                    witx_confg = Some(WitxConf::Paths(paths));
                }
            }
        }
        let bc = BlocklessConfig {
            c: witx_confg.take().expect("witx is not set."),
            link_method: link_method.expect("link_method is not set."),
            target: target.take().expect("target is not set."),
        };
        Ok(bc)
    }

    pub fn load_document(&self) -> witx::Document {
        self.c.load_document()
    }
}

impl Parse for BlocklessConfig {
    fn parse(input: ParseStream) -> Result<Self> {
        let contents;
        let _ = braced!(contents in input);
        let fields: Punctuated<BlocklessConfigField, Token![,]> =
            contents.parse_terminated(BlocklessConfigField::parse)?;
        Self::build(fields.into_iter())
    }
}

impl Parse for BlocklessConfigField {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::witx) {
            input.parse::<kw::witx>()?;
            input.parse::<Token![:]>()?;
            Ok(BlocklessConfigField::WitxField(input.parse()?))
        } else if lookahead.peek(kw::target) {
            input.parse::<kw::target>()?;
            input.parse::<Token![:]>()?;
            Ok(BlocklessConfigField::TargetField(input.parse()?))
        } else if lookahead.peek(kw::link_method) {
            input.parse::<kw::link_method>()?;
            input.parse::<Token![:]>()?;
            Ok(BlocklessConfigField::LinkMethodField(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}
