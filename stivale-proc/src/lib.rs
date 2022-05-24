extern crate proc_macro;

use proc_macro::TokenStream;

/// The header structure needs to reside in the `.stivale2hdr` ELF section
/// in order for the bootloader to find it. The use of this macro instructs
/// the compiler to put the following structure in the said section.
///
/// ## Usage
/// ```rust,norun
/// #[macro_use]
/// extern crate stivale_boot;
///
/// use stivale_boot::v2::StivaleHeader;
///
/// #[stivale2hdr]
/// static STIVALE_HDR: StivaleHeader = StivaleHeader::new();
/// ```
#[proc_macro_attribute]
pub fn stivale2hdr(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStatic);
    let ty = &input.ty;

    quote::quote! {
        // ensures that the type of the header is `v2::StivaleHeader`.
        const _: () = { fn __sheader_ty_chk(e: #ty) -> ::stivale_boot::v2::StivaleHeader { e } };

        #[link_section = ".stivale2hdr"]
        #[no_mangle]
        #[used]
        #input
    }
    .into()
}
