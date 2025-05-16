use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Errô" => "Err",
        "Bão" => "Ok",
        "Texto" => "String",
        "Dicionário" => "HashMap",
        "Padrão" => "Default",
        "Erro" => "Error",
        "Opção" => "Option",
        "Algum" => "Some",
        "Nenhum" => "None",
        "Resultado" => "Result",
        "Eu" => "Self",
        "printa_quebra_linha" => "println",
        "quebra" => "break",
        "assíncrono" => "async",
        "espera" => "await",
        "vai_na_fé" => "loop",
        "mova" => "move",
        "trem" => "crate",
        "código_inalcançável" => "unreachable_code",
        "tipo" => "as",
        "constante" => "const",
        "convenção" => "trait",
        "trem_perigoso" => "unsafe",
        "em" => "in",
        "de" => "from",
        "dinâmico" => "dyn",
        "confia_que_tem" => "unwrap",
        "padrão" => "default",
        "como_ref" => "as_ref",
        "es" => "io",
        "externo" => "extern",
        "falso" => "false",
        "função" => "fn",
        "insere" => "insert",
        "pegue" => "get",
        "permita" => "allow",
        "merda" | "fudeo" => "panic",
        "módulo" => "mod",
        "mutável" => "mut",
        "novinho" => "new",
        "onde" => "where",
        "para" => "for",
        "pegue_ou_insira_em" => "get_or_insert_with",
        "principal" => "main",
        "público" => "pub",
        "que?" => None?,
        "retorne" => "return",
        "implementa" => "impl",
        "cheque" => "match",
        "se" => "if",
        "senão" => "else",
        "eu" => "self",
        "defina" => "let",
        "estático" => "static",
        "estrutura" => "struct",
        "espere" => "expect",
        "enquanto" => "while",
        "em" => "into",
        "verdadeiro" | "verídico" => "true",
        "enumeração" => "enum",
        "Grupo" => "Group",
        "Identificador" => "Ident",
        "TremDeTokens" => "TokenStream",
        "ÁrvoreDeTokens" => "TokenTree",
        "para_texto" => "to_string",
        "como_texto" => "as_str",
        "Vetor" => "Vec",
        "fluxo" => "stream",
        "empurre" => "push",
        "extenda" => "extend",
        "delimite" => "delimiter",
        "Pontuação" => "Punct",
        "macro_procedural" => "proc_macro",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn ferrugem(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
