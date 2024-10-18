// // Use regular expression replacements (stack program for reveal) to strip all zkay specific language features
// // so that code can be passed to solc for type checking.

// import re
use fancy_regex::{Captures, Regex as Regexf};
use lazy_static::lazy_static;
use regex::Regex;
use zkay_config::config::CFG;
// // Declaration for me which is injected into each contract
pub const ME_DECL: &str = " address private me = msg.sender;";

// // ---------  Lexer Rules ---------
pub const WS_PATTERN: &str = r"[ \t\r\n\u000C]";
pub const ID_PATTERN: &str = r"[a-zA-Z\$_][a-zA-Z0-9\$_]*";
pub const UINT_PATTERN: &str = r"uint|uint8|uint16|uint24|uint32|uint40|uint48|uint56|uint64|uint72|uint80|uint88|uint96|uint104|uint112|uint120|uint128|uint136|uint144|uint152|uint160|uint168|uint176|uint184|uint192|uint200|uint208|uint216|uint224|uint232|uint240|uint248|uint256";
pub const INT_PATTERN: &str = r"int|int8|int16|int24|int32|int40|int48|int56|int64|int72|int80|int88|int96|int104|int112|int120|int128|int136|int144|int152|int160|int168|int176|int184|int192|int200|int208|int216|int224|int232|int240|int248|int256";
pub const HOMOMORPHISM_PATTERN: &str = r"<\+?>";
pub const NONID_START: &str = r"(?:[^a-zA-Z0-9\$_]|^)";
pub const NONID_END: &str = r"(?:[^a-zA-Z0-9\$_]|$)";
lazy_static! {
//   static ref USER_TYPE_PATTERN: String = format!("(?:(?:{ID_PATTERN}\\.)*(?:{ID_PATTERN}))");
  static ref ELEM_TYPE_PATTERN: String =  format!("(?:address|address payable|bool|{UINT_PATTERN}|{INT_PATTERN}|(?:(?:{ID_PATTERN}\\.)*(?:{ID_PATTERN})))");
  static ref PARENS_PATTERN: Regex =  Regex::new(r"[()]").unwrap();
  static ref BRACE_PATTERN: Regex =  Regex::new(r"[{}]").unwrap();
  static ref STRING_OR_COMMENT_PATTERN: Regexf= Regexf::new(
   r#"^(?P<repl>
   (?://[^\r\n]*)                           #  match line comment
   |(?:/\*.*?\*/)                           #  match block comment
   |(?:(?<=')(?:[^'\r\n\\]|(?:\\.))*(?='))  #  match single quoted string literal
   |(?:(?<=")(?:[^"\r\n\\]|(?:\\.))*(?="))  #  match double quoted string literal
   )$"#).unwrap();//.expect("RegexSetBuilder build fail");



// ---------  Parsing ---------
  static ref CONTRACT_START_PATTERN : Regexf =  Regexf::new(&format!("{NONID_START}contract{WS_PATTERN}*{ID_PATTERN}{WS_PATTERN}*(?=[{{])")).unwrap();
// Regex to match annotated types

static ref ATYPE_PATTERN : Regex =  Regex::new(&format!(r"(?P<keep>{NONID_START}{}{WS_PATTERN}*)(?P<repl>@{WS_PATTERN}*{ID_PATTERN}({HOMOMORPHISM_PATTERN})?)",*ELEM_TYPE_PATTERN)).unwrap();
       // match basic type
  // match @owner[<op>]

// Regexes to match "all" and "final"
//   pub const  MATCH_WORD_FSTR : &str = "(?P<keep>{NONID_START})(?P<repl>{{}})(?={NONID_END})";
  static ref FINAL_PATTERN : Regexf =  Regexf::new(&format!("(?P<keep>{NONID_START})(?P<repl>final)(?={NONID_END})")).unwrap();
  static ref ALL_PATTERN : Regexf =  Regexf::new(&format!("(?P<keep>{NONID_START})(?P<repl>all)(?={NONID_END})")).unwrap();

// Pragma regex
  static ref PRAGMA_PATTERN : Regex =  Regex::new(&format!(r"(?P<keep>{NONID_START}pragma\s*)(?P<repl>zkay.*?);")).unwrap();

// Regex to match tagged mapping declarations
  static ref MAP_PATTERN : Regexf =  Regexf::new(&format!(
    "(?P<keep>{NONID_START}mapping{WS_PATTERN}*\\({WS_PATTERN}*{}{WS_PATTERN}*)(?P<repl>!{WS_PATTERN}*{ID_PATTERN})(?={WS_PATTERN}*=>{WS_PATTERN}*)",*ELEM_TYPE_PATTERN)).unwrap();  //  # match 'mapping (address'   # match '!tag'  #expect '=>'

// Regex to detect start of reveal
  static ref REVEAL_START_PATTERN : Regexf =  Regexf::new(&format!("(?:^|(?<=[^\\w]))reveal{WS_PATTERN}*(?=\\()")).unwrap();  // match "reveal", expect "("

// Regex to detect addhom & unhom
  static ref ADDHOM_UNHOM_PATTERN : Regexf =  Regexf::new(&format!("(?:^|(?<=[^\\w]))(?P<repl>addhom|unhom){WS_PATTERN}*(?=\\()")).unwrap();
}
// """
// Preserve newlines and replace all other characters with spaces
// :return whitespace string with same length as instr and with the same line breaks
// """
pub fn create_surrogate_string(instr: &str) -> String {
    instr
        .chars()
        .map(|e| if e == '\n' { '\n' } else { ' ' })
        .collect()
}

pub fn find_matching_parenthesis(code: &str, open_parens_loc: i32) -> i32 {
    // """
    // Get index of matching parenthesis/bracket/brace.
    // :param code: code in which to search
    // :param open_parens_loc: index of the opening parenthesis within code
    // :return: index of the matching closing parenthesis
    // """

    // Determine parenthesis characters
    let open_sym = code.as_bytes()[open_parens_loc as usize] as char;
    let close_sym;
    if open_sym == '(' {
        close_sym = ')'
    } else if open_sym == '{' {
        close_sym = '}'
    } else if open_sym == '[' {
        close_sym = ']'
    } else {
        // close_sym = ' ';
        // raise ValueError("Unsupported parenthesis type")
        panic!("Unsupported parenthesis type");
    }

    let pattern = Regex::new(&format!("[{open_sym}{close_sym}]")).unwrap();
    let mut idx = open_parens_loc + 1;
    let mut open = 1;
    while open > 0 {
        let cstr = &code[idx as usize..];
        idx += pattern.find(cstr).unwrap().start() as i32;
        open += if code.as_bytes()[idx as usize] as char == open_sym {
            1
        } else {
            -1
        };
        idx += 1;
    }
    idx - 1
}

// Replacing reveals only with regex is impossible because they could be nested -> do it with a stack
// """Replace reveal expressions by their inner expression, with whitespace padding."""
pub fn strip_reveals(code: &str) -> String {
    let mut code = code.to_owned();
    let c = code.clone();
    let matches = REVEAL_START_PATTERN.find_iter(&c);
    for m in matches {
        let before_reveal_loc = m.clone().expect("m").start();
        let reveal_open_parens_loc = m.clone().expect("m").end();

        // Find matching closing parenthesis
        let reveal_close_parens_loc =
            find_matching_parenthesis(&code, reveal_open_parens_loc as i32) as usize;

        // Go backwards to find comma before owner tag
        let last_comma_loc = code[..reveal_close_parens_loc].rfind(",").unwrap();

        // Replace reveal by its inner expression + padding
        code = format!(
            "{}{}{}{}{}",
            &code[..before_reveal_loc],
            create_surrogate_string(&code[before_reveal_loc..reveal_open_parens_loc]),
            &code[reveal_open_parens_loc..last_comma_loc],
            create_surrogate_string(&code[last_comma_loc..reveal_close_parens_loc]),
            &code[reveal_close_parens_loc..]
        );
    }
    code
}
// """Add an additional address me = msg.sender state variable declaration right before the closing brace of each contract definition."""
pub fn inject_me_decls(code: &str) -> String {
    let matches = CONTRACT_START_PATTERN.find_iter(code);
    let mut insert_indices = vec![];
    for m in matches {
        insert_indices.push(find_matching_parenthesis(
            code,
            m.clone().unwrap().end() as i32,
        ));
    }
    let parts: Vec<_> = [0]
        .iter()
        .chain(&insert_indices)
        .zip(insert_indices.iter().chain([&(code.len() as i32)]))
        .map(|(&i, &j)| code[i as usize..j as usize].to_owned())
        .collect();
    parts.join(ME_DECL)
}

// """
// Replace all occurrences of search_pattern with capture group <keep> (if any) + replacement.

// Replacement is either
//     a) replacement_fstr (if replacement_fstr does not contain "{}")
//     b) replacement_fstr with {} replaced by whitespace corresponding to content of capture group <repl>
//        (such that replacement length == <repl> length with line breaks preserved)

// The <repl> capture group must be the last thing that is matched in search pattern
// """
// pub fn replace_with_surrogates(
//     code: &str,
//     search_pattern: &RegexSet,
//     replacement_fstr: &str,
// ) -> String {
//     // Compile each pattern independently.
//     let regexes: Vec<_> = search_pattern
//         .patterns()
//         .iter()
//         .map(|pat| Regex::new(pat).unwrap())
//         .collect();

//     // Match against the whole set first and identify the individual
//     // matching patterns.
//     search_pattern
//         .matches(code)
//         .into_iter()
//         // Dereference the match index to get the corresponding
//         // compiled pattern.
//         .map(|index| &regexes[index])
//         // To get match locations or any other info, we then have to search the
//         // exact same haystack again, using our separately-compiled pattern.
//         .fold(code.to_owned(), |a, re| {
//             replace_with_surrogate(&a, re, replacement_fstr)
//         })
// }
pub fn replace_with_surrogate(
    code: &str,
    search_pattern: &Regex,
    replacement_fstr: &str,
) -> String {
    if !replacement_fstr.is_empty() {
        println!(
            "=====!replacement_fstr.is_empty()===={}====================={search_pattern:?}",
            search_pattern.as_str()
        );
    }
    let mut code = code.to_owned();
    let keep_repl_pattern = if search_pattern.as_str().contains("(?P<keep>") {
        r"$keep"
    } else {
        ""
    };
    let has_ph = replacement_fstr.is_empty();
    let mut replacement = replacement_fstr.to_owned();
    let mut search_idx = 0;
    let mut c;
    let mut flag = true;
    for _ in 0..100 {
        c = code.clone();
        let matches = search_pattern.captures(&c[search_idx..]);
        // if PRAGMA_PATTERN.as_str() == search_pattern.as_str() {
        //     println!("=======matches====={search_idx}====={replacement}====={matches:?}");
        // }
        if matches.is_none() {
            flag = false;
            break;
        }
        let end = matches.as_ref().unwrap().get(0).unwrap().end();
        if has_ph {
            let repl = matches
                .and_then(|cap| cap.name("repl").map(|repl| repl.as_str()))
                .unwrap();
            replacement = create_surrogate_string(repl);
        }
        // if PRAGMA_PATTERN.as_str() == search_pattern.as_str() {
        //     println!(
        //         "====code===={}======={code:?}",
        //         keep_repl_pattern.to_owned() + &replacement
        //     );
        // }
        code = code[..search_idx].to_owned()
            + &search_pattern.replace(
                &code[search_idx..],
                keep_repl_pattern.to_owned() + &replacement,
            );
        // if PRAGMA_PATTERN.as_str() == search_pattern.as_str() {
        //     println!("=after==code==={search_idx}========{code:?}");
        // }
        search_idx += end + 1;
    }
    if flag {
        println!("-=====exit loop exception=================");
    }
    code
}

pub fn replace_with_surrogatef(
    code: &str,
    search_pattern: &Regexf,
    replacement_fstr: &str,
) -> String {
    let mut code = code.to_owned();
    let keep_repl_pattern = if search_pattern.as_str().contains("(?P<keep>") {
        r"keep"
    } else {
        ""
    };
    let has_ph = replacement_fstr.is_empty();
    let mut replacement = replacement_fstr.to_owned();
    let mut search_idx = 0;
    let mut c;
    let mut flag = true;
    for _ in 0..1000 {
        // println!("===code.len()========={}",code.len());
        c = code.clone();
        let matches = search_pattern
            .captures(&c[search_idx..])
            .expect("Error running regex");
        // if search_pattern.as_str()==MAP_PATTERN.as_str(){
        //     println!("=matches=========={matches:?}=======");
        // }
        if matches.is_none() {
            flag = false;
            // println!("===========matches.is_none()=======");
            break;
        }
        let end = matches.as_ref().unwrap().get(0).unwrap().end();
        if has_ph {
            let repl = matches
                .and_then(|cap| cap.name("repl").map(|repl| repl.as_str()))
                .unwrap();
            replacement = create_surrogate_string(repl);
            // println!("===replacement========{replacement}=======");
        }

        code = code[..search_idx].to_owned()
            + &search_pattern.replace(&code[search_idx..], |caps: &Captures| {
                format!(
                    "{}{}",
                    if keep_repl_pattern.is_empty() {
                        keep_repl_pattern
                    } else {
                        &caps[keep_repl_pattern]
                    },
                    replacement
                )
            });
        // if search_pattern.as_str()==MAP_PATTERN.as_str(){
        //     println!("====end========{end},======={code}======");
        // }
        search_idx += end + 1;
    }
    if flag {
        println!("===========err exit loop=======");
    }
    code
}
// """
// Returns the solidity code to which the given zkay_code corresponds when dropping all privacy features,
// while preserving original formatting
// """
pub fn fake_solidity_code(code: &str) -> String {
    // Strip string literals and comments
    let mut code = replace_with_surrogatef(code, &STRING_OR_COMMENT_PATTERN, "");

    // Replace zkay pragma with solidity pragma
    code = replace_with_surrogate(
        &code,
        &PRAGMA_PATTERN,
        &format!(
            " solidity {};",
            CFG.lock().unwrap().zkay_solc_version_compatibility()
        ),
    );

    // Strip final
    code = replace_with_surrogatef(&code, &FINAL_PATTERN, "");

    // Strip ownership annotations
    code = replace_with_surrogate(&code, &ATYPE_PATTERN, "");

    // println!("===={}====before====={code}",MAP_PATTERN.as_str());
    // Strip map key tags
    code = replace_with_surrogatef(&code, &MAP_PATTERN, "");
    // println!("======after======={code}");

    // Strip addhom / unhom expressions
    code = replace_with_surrogatef(&code, &ADDHOM_UNHOM_PATTERN, "");

    // Strip reveal expressions
    code = strip_reveals(&code);

    // Inject me address declaration (should be okay for type checking, maybe not for program analysis)
    // An alternative would be to replace me by msg.sender, but this would affect code length (error locations)
    code = inject_me_decls(&code);

    format!("//SPDX-License-Identifier: UNLICENSED\n{code}")
}
