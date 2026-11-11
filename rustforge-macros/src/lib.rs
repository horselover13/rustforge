//! Attribute macros for RustForge.

use proc_macro::{Delimiter, TokenStream, TokenTree};

/// Marks a function as the mod's entry point.
///
/// `hellforge_mod_init` installs the HellForge API and then calls the
/// annotated function, on the loader's thread, once per set-up. The function
/// must be safe and take either no arguments or exactly one
/// `&'static rustforge::hf_api::HellForgeApi` argument.
///
/// ```ignore
/// use rustforge::hf_entry;
///
/// #[hf_entry]
/// fn init(api: &rustforge::hf_api::HellForgeApi) {
///     api.log("hello from RustForge");
/// }
/// ```
///
/// Only one function should be marked per DLL. If several are, the first one
/// registered wins.
#[proc_macro_attribute]
pub fn hf_entry(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return compile_error("#[hf_entry] does not take arguments");
    }

    match expand(item.clone()) {
        Ok(generated) => {
            let mut output = item;
            output.extend(generated);
            output
        }
        Err(message) => compile_error(&message),
    }
}

fn expand(item: TokenStream) -> Result<TokenStream, String> {
    let info = FnInfo::parse(&item)?;

    let (param, call) = match info.arity {
        0 => ("_api", format!("{}();", info.name)),
        1 => ("api", format!("{}(api);", info.name)),
        arity => {
            return Err(format!(
                "#[hf_entry] functions must take no arguments or one `&HellForgeApi` argument \
                 (found {arity} arguments)"
            ));
        }
    };

    let init_fn = format!("__rustforge_entry_init_{}", info.name);
    let init_static = format!("__RUSTFORGE_ENTRY_INIT_{}", info.name);

    let generated = format!(
        r##"
#[doc(hidden)]
#[allow(non_snake_case)]
extern "C" fn {init_fn}() {{
    ::rustforge::hf_api::__register_entry(
        |{param}: &'static ::rustforge::hf_api::HellForgeApi| {{
            {call}
        }},
    );
}}

#[allow(non_upper_case_globals)]
#[used]
#[cfg_attr(target_os = "windows", unsafe(link_section = ".CRT$XCU"))]
#[cfg_attr(
    any(
        target_os = "linux",
        target_os = "android",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "illumos",
        target_os = "solaris",
        target_os = "haiku",
    ),
    unsafe(link_section = ".init_array")
)]
#[cfg_attr(target_vendor = "apple", unsafe(link_section = "__DATA,__mod_init_func"))]
static {init_static}: extern "C" fn() = {init_fn};

#[cfg(not(any(
    target_os = "windows",
    any(
        target_os = "linux",
        target_os = "android",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "illumos",
        target_os = "solaris",
        target_os = "haiku",
    ),
    target_vendor = "apple",
)))]
compile_error!("#[hf_entry] is not supported on this target");
"##
    );

    generated
        .parse()
        .map_err(|_| "failed to generate the #[hf_entry] registration".to_string())
}

struct FnInfo {
    name: String,
    arity: usize,
}

impl FnInfo {
    fn parse(item: &TokenStream) -> Result<Self, String> {
        let tokens: Vec<TokenTree> = item.clone().into_iter().collect();
        let mut index = 0;

        while index < tokens.len() {
            // Skip outer attributes: `#` followed by `[...]` or `![...]`.
            if matches!(&tokens[index], TokenTree::Punct(punct) if punct.as_char() == '#') {
                index += 2;
                continue;
            }

            if let TokenTree::Ident(ident) = &tokens[index] {
                match ident.to_string().as_str() {
                    "async" => {
                        return Err("#[hf_entry] cannot be used on an async function".to_string());
                    }
                    "unsafe" => {
                        return Err(
                            "#[hf_entry] cannot be used on an unsafe function; keep the \
                             function safe and put unsafe code in its body"
                                .to_string(),
                        );
                    }
                    "fn" => {
                        let Some(TokenTree::Ident(name)) = tokens.get(index + 1) else {
                            return Err("#[hf_entry] must be applied to a function".to_string());
                        };

                        let name = name.to_string();
                        let name = name.strip_prefix("r#").unwrap_or(&name).to_string();
                        let arity = find_arity(&tokens[index + 2..])?;

                        return Ok(Self { name, arity });
                    }
                    _ => {}
                }
            }

            index += 1;
        }

        Err("#[hf_entry] can only be used on a function".to_string())
    }
}

/// Finds the function's parameter list, skipping generic parameters.
fn find_arity(tokens: &[TokenTree]) -> Result<usize, String> {
    let mut index = 0;

    if matches!(tokens.first(), Some(TokenTree::Punct(punct)) if punct.as_char() == '<') {
        let mut depth = 0isize;

        while index < tokens.len() {
            if let TokenTree::Punct(punct) = &tokens[index] {
                match punct.as_char() {
                    '<' => depth += 1,
                    '>' => {
                        depth -= 1;
                        if depth == 0 {
                            index += 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }

            index += 1;
        }
    }

    while index < tokens.len() {
        if let TokenTree::Group(group) = &tokens[index] {
            if group.delimiter() == Delimiter::Parenthesis {
                return Ok(count_params(group.stream()));
            }
        }

        index += 1;
    }

    Err("could not find the function's parameter list".to_string())
}

fn count_params(stream: TokenStream) -> usize {
    let mut depth = 0usize;
    let mut commas = 0usize;
    let mut any = false;

    for token in stream {
        any = true;

        if let TokenTree::Punct(punct) = token {
            match punct.as_char() {
                '<' | '(' | '[' => depth += 1,
                '>' | ')' | ']' => depth = depth.saturating_sub(1),
                ',' if depth == 0 => commas += 1,
                _ => {}
            }
        }
    }

    if any { commas + 1 } else { 0 }
}

fn compile_error(message: &str) -> TokenStream {
    format!("compile_error!({message:?});")
        .parse()
        .expect("compile_error! token stream should parse")
}
