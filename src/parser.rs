use regex::Regex;
use std::fmt;

#[derive(Debug)]
pub enum Token {
    Header {
        level: u8,
        text: String,
    },
    Paragraph {
        text: String,
    },
    UListItem {
        text: String,
    },
    OLStart {},
    OLEnd {},
    OListItem {
        text: String,
    },
    SimpleText {
        text: String,
    },
    InlineCode {
        text: String,
    },
    Quote {
        text: String,
        nested_token: Box<Token>,
    },
    Bold {
        text: String,
    },
    Italic {
        text: String,
    },
    Strikethrough {
        text: String,
    },
    // CodeBlock {
    //     text: String,
    // },
    Link {
        text: String,
        url: String,
    },
    CodeBlockStart {},
    CodeBlockEnd {},
    HorizLine {},
    BreakLine {},
    None {},
}

trait HasText {
    fn text(&self) -> Option<&str>;
}

impl HasText for Token {
    fn text(&self) -> Option<&str> {
        match self {
            Token::Header { text, .. }
            | Token::Paragraph {text, ..}
            | Token::UListItem {text, ..}
            | Token::OListItem {text, ..}
            | Token::SimpleText {text, ..}
            | Token::InlineCode {text, ..}
            | Token::Quote {text, ..}
            | Token::Bold {text, ..}
            | Token::Italic {text, ..}
            | Token::Strikethrough {text, ..}
            | Token::Link {text, ..}
            // Add more as needed
            => Some(text),
            _ => None,
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::Header { level, text } => Token::Header {
                level: *level,
                text: text.clone(),
            },
            Token::Paragraph { text } => Token::Paragraph { text: text.clone() },
            Token::UListItem { text } => Token::UListItem { text: text.clone() },
            Token::OListItem { text } => Token::OListItem { text: text.clone() },
            Token::InlineCode { text } => Token::InlineCode { text: text.clone() },
            Token::SimpleText { text } => Token::SimpleText { text: text.clone() },
            Token::Quote { text, nested_token } => Token::Quote {
                text: text.clone(),
                nested_token: nested_token.clone(),
            },
            Token::Bold { text } => Token::Bold { text: text.clone() },
            Token::Italic { text } => Token::Italic { text: text.clone() },
            Token::Strikethrough { text } => Token::Strikethrough { text: text.clone() },
            // Token::CodeBlock { text } => Token::CodeBlock { text: text.clone() },
            Token::Link { text, url } => Token::Link {
                text: text.clone(),
                url: url.clone(),
            },
            Token::OLStart {} => Token::OLStart {},
            Token::OLEnd {} => Token::OLEnd {},
            Token::CodeBlockStart {} => Token::CodeBlockStart {},
            Token::CodeBlockEnd {} => Token::CodeBlockEnd {},
            Token::HorizLine {} => Token::HorizLine {},
            Token::BreakLine {} => Token::BreakLine {},
            Token::None {} => Token::None {},
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Header { level, text } => write!(f, "<h{0}>{1}</h{0}>", level, text),
            Token::Paragraph { text } => write!(f, "<p>{}</p>", text),
            Token::UListItem { text } => write!(f, "<li>{}</li>", text),
            Token::OListItem { text } => write!(f, "<li>{}</li>", text),
            Token::InlineCode { text } => write!(f, "<code>{}</code>", text),
            Token::Bold { text } => write!(f, "<strong>{}</strong>", text),
            Token::Italic { text } => write!(f, "<em>{}</em>", text),
            Token::Quote { text, nested_token } => {
                write!(f, "<q>{}{}</q>", text, nested_token.as_ref())
            }
            Token::OLStart {} => write!(f, "<ol>"),
            Token::OLEnd {} => write!(f, "</ol>"),
            Token::Strikethrough { text } => write!(f, "<s>{}</s>", text),
            //Token::CodeBlock { text } => write!(f, "<pre><code>{}</code></pre>", text),
            Token::CodeBlockStart {} => write!(f, "<pre><code>"),
            Token::CodeBlockEnd {} => write!(f, "</code></pre>"),
            Token::Link { text, url } => write!(f, "<a href=\"{}\">{}</a>", url, text),
            Token::SimpleText { text } => write!(f, "{}", text),
            Token::HorizLine {} => write!(f, "<hr>"),
            Token::BreakLine {} => write!(f, "<br/>"),
            _ => write!(f, ""),
        }
    }
}

fn convert_inline_markdown(line: &mut String) -> String {
    // Treating bold syntax
    let mut re = Regex::new(r"\*\*(.+?)\*\*").unwrap();
    let mut resulted_format = String::new();
    while re.is_match(line) {
        // Find the indexes of the first match
        let found_ind = re.find(line).unwrap();
        let mut found_substring = (&line[found_ind.range()]).to_string(); // **bolded word**
        let start_of_string = (&line[0..found_ind.start()]).to_string(); // Whatever is before it

        // Removes the starting ** and the ending **
        found_substring.replace_range(0..2, "");
        found_substring.replace_range(found_substring.len() - 2..found_substring.len(), "");

        // Now found_substring contains only the bolded text so we can make it html
        resulted_format = format!(
            "{resulted_format}{}<strong>{}</strong>",
            start_of_string, found_substring
        );

        // We then trim the start of the original line till what we found
        let _ = &line.replace_range(..found_ind.end(), "");
    }
    resulted_format = resulted_format + line;

    // Treating italic - NEEDS TO BE DONE AFTER BOLD
    re = Regex::new(r"\*(.+?)\*").unwrap();
    let mut line_copy = resulted_format.clone();
    resulted_format = String::new();
    while re.is_match(&line_copy) {
        // Find the indexes of the first match
        let found_ind = re.find(&line_copy).unwrap();
        let mut found_substring = (&line_copy[found_ind.range()]).to_string(); // *italic word*
        let start_of_string = (&line_copy[0..found_ind.start()]).to_string(); // Whatever is before it

        // Removes the starting * and the ending *
        found_substring.remove(0);
        found_substring.remove(found_substring.len() - 1);

        // Now found_substring contains only the italic text so we can make it html
        resulted_format = format!(
            "{resulted_format}{}<i>{}</i>",
            start_of_string, found_substring
        );

        // We then trim the start of the original line till what we found
        let _ = &line_copy.replace_range(..found_ind.end(), "");
    }
    resulted_format = resulted_format + &line_copy;

    // Treating LINK text searches for []() pair allows nesting for the [[]]
    re = Regex::new(r"\[[^\[\]]*(?:\[[^\[\]]*\][^\[\]]*)*\]\([^()]*\)").unwrap();
    let mut line_copy = resulted_format.clone();
    resulted_format = String::new();

    while re.is_match(&line_copy) {
        //println!("[debug] Found a link pattern in line:\n{}\n", line_copy);
        // Find the indexes of the first match
        let found_ind = re.find(&line_copy).unwrap();
        let found_substring = (&line_copy[found_ind.range()]).to_string(); // [Link Text](url)
        let start_of_string = (&line_copy[0..found_ind.start()]).to_string(); // Whatever is before it

        let text_part_re = Regex::new(r"\[[^\[\]]*(?:\[[^\[\]]*\][^\[\]]*)*\]").unwrap();
        let text_part_range = text_part_re.find(&found_substring).unwrap();
        let link_text = (&found_substring[1..text_part_range.end() - 1]).to_string(); // The title of the link
        let mut link_url =
            (&found_substring[text_part_range.end()..found_substring.len()]).to_string(); // Still contains the '('  ')'

        link_url.remove(0);
        link_url.remove(link_url.len() - 1);

        // Now found_substring contains only the italic text so we can make it html
        resulted_format = format!(
            "{resulted_format}{}<a href=\"{}\">{}</a>",
            start_of_string, link_url, link_text
        );

        // We then trim the start of the original line till what we found
        let _ = &line_copy.replace_range(..found_ind.end(), "");
    }
    resulted_format = resulted_format + &line_copy;

    return resulted_format;
}

pub fn tokenize_text(str_iter: impl Iterator<Item = String>) -> Vec<String> {
    let mut string_result: Vec<String> = Vec::new();
    let input_text: Vec<String> = str_iter.collect();

    let mut token_list: Vec<Token> = Vec::new();
    let mut token_list_processed = Vec::new();

    // let mut debug_print_str: String = "[ ".to_string();
    for line in &input_text {
        let token = tokenize_line(line.clone()).unwrap_or(Token::BreakLine {});

        // Debug printing
        // debug_print_str += &format!("{:?},\t", token);

        token_list.push(token);
    }
    // debug_print_str += " ]\n";
    // let debugRe = Regex::new(r"\{[^{}]*\}").unwrap();
    // println!("[DEBUG] {}", debugRe.replace_all(&debug_print_str, ""));

    // Parse further the token_list for tokens which need multi-line support
    // eg. code blocks, ordered lists
    // We add special tokens that will "encapsulate" the content that requires multi-line support
    let mut last_token = Token::None {};
    let mut inside_code_block = false;
    for (i, token) in token_list.iter().enumerate() {
        // PUSH ANYTHING BEFORE THE CURRENT TOKEN
        if matches!(token, Token::OListItem { .. })
            && !matches!(last_token, Token::OListItem { .. })
        {
            // If this is one of the first series of ol's then we add starting tag
            token_list_processed.push(Token::OLStart {});
        }

        if matches!(token, Token::CodeBlockEnd {}) {
            inside_code_block = false;
        }

        // PUSH THE CURRENT TOKEN
        token_list_processed.push(token.clone());

        // Code blocks remove all formatting inside so we'll only use simple_text tokens
        if inside_code_block {
            let raw_line = input_text[i].clone();
            token_list_processed.pop();
            token_list_processed.push(Token::SimpleText { text: raw_line });
        }

        // PUSH ANYTHING AFTER THE CURRENT TOKEN
        if !matches!(token, Token::OListItem { .. })
            && matches!(last_token, Token::OListItem { .. })
        {
            // If this is isn't an ol item but last item was one, we add ending tag
            token_list_processed.push(Token::OLEnd {});
        }

        if matches!(token, Token::CodeBlockStart {}) {
            inside_code_block = true;
        }

        last_token = token.clone();
    }

    for token in token_list_processed {
        string_result.push(token.to_string());
    }

    string_result
}

/// Tokenizes a single line of text.
///
/// This function takes a markdown string as input and processes it to extract tokens
/// based on the implemented parsing logic. It is typically used as part of
/// a larger markdown parsing pipeline.
///
/// # Arguments
///
/// * `text` - A `String` containing the line of markdown text to be tokenized.
///
/// # Example
///
/// ```rust
/// let line = String::from("This is a sample line.");
/// tokenize_line(line);
/// ```
pub fn tokenize_line(line: String) -> Result<Token, ()> {
    let token_result;
    let mut line_copy = line;

    // Not all blocks accept the inline parsing, eg. Code blocks

    // Line is a Header
    if line_copy.starts_with("#") {
        let mut count = 0;
        while line_copy.starts_with("#") {
            count += 1;
            line_copy.remove(0);
        }

        let inline_converted_line = convert_inline_markdown(&mut line_copy);

        token_result = Token::Header {
            level: (count),
            text: (inline_converted_line),
        };
        return Ok(token_result);
    }

    // Line is a block quote
    if line_copy.starts_with(">") {
        line_copy.remove(0);
        let inline_converted_line = convert_inline_markdown(&mut line_copy);
        let nested_token = tokenize_line(inline_converted_line).unwrap();
        match nested_token {
            Token::Paragraph { text } => {
                token_result = Token::Quote {
                    text: (text),
                    nested_token: Box::new(Token::None {}),
                }
            }
            _ => {
                token_result = Token::Quote {
                    text: ("[DEBUG]").to_owned(),
                    nested_token: Box::new(nested_token),
                }
            }
        }

        return Ok(token_result);
    }

    // Line is 3 dashes -> HorizLine
    if line_copy.trim() == "---" {
        token_result = Token::HorizLine {};
        return Ok(token_result);
    }

    // Line followed by a space is a ListItem
    if line_copy.starts_with(&['-', '*', '+']) && line_copy.chars().nth(1).unwrap_or('.') == ' ' {
        line_copy.remove(0);
        let inline_converted_line = convert_inline_markdown(&mut line_copy);
        token_result = Token::UListItem {
            text: (inline_converted_line),
        };
        return Ok(token_result);
    }

    // Line is empty -> BreakLine
    if line_copy == "" {
        token_result = Token::BreakLine {};
        return Ok(token_result);
    }

    // Line is a codeblock end cannot follow text (ordering matters)
    if line_copy.trim_end().trim() == "```" {
        token_result = Token::CodeBlockEnd {};
        return Ok(token_result);
    }

    // Line is codeblock start can follow text after it
    if line_copy.starts_with("```") {
        token_result = Token::CodeBlockStart {};
        return Ok(token_result);
    }

    // Line starts with a number followed by a . and a space
    let re = Regex::new(r"^\d+\.\s").unwrap();
    if re.is_match(&line_copy) {
        let re_res = re.find(&line_copy).unwrap();
        let inlined_text = convert_inline_markdown(&mut line_copy.split_off(re_res.end()));
        token_result = Token::OListItem {
            text: (inlined_text),
        };

        return Ok(token_result);
    }

    // If none of the above -> Paragraph
    let inline_converted_line = convert_inline_markdown(&mut line_copy);
    token_result = Token::Paragraph {
        text: (inline_converted_line),
    };
    Ok(token_result)
}
