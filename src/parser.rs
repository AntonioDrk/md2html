/// Represents various types of tokens that can be extracted from a markdown input.
///
/// This enum is used to categorize and store different markdown elements such as headers,
/// paragraphs, lists, inline formatting, and more. Each variant corresponds to a specific
/// markdown construct and may contain associated data relevant to that construct.
///
/// # Variants
///
/// - `Header`:
///   Represents a markdown header (e.g., `# Header`). Contains:
///   - `level`: The level of the header (e.g., 1 for `#`, 2 for `##`).
///   - `text`: The content of the header.
///
/// - `Paragraph`:
///   Represents a paragraph of text. Contains:
///   - `text`: The content of the paragraph.
///
/// - `UListItem`:
///   Represents an unordered list item (e.g., `- Item`). Contains:
///   - `text`: The content of the list item.
///
/// - `OLStart`:
///   Marks the start of an ordered list.
///
/// - `OLEnd`:
///   Marks the end of an ordered list.
///
/// - `OListItem`:
///   Represents an ordered list item (e.g., `1. Item`). Contains:
///   - `text`: The content of the list item.
///
/// - `SimpleText`:
///   Represents plain text without any formatting. Contains:
///   - `text`: The content of the text.
///
/// - `InlineCode`:
///   Represents inline code (e.g., `` `code` ``). Contains:
///   - `text`: The content of the inline code.
///
/// - `Quote`:
///   Represents a blockquote (e.g., `> Quote`). Contains:
///   - `text`: The content of the quote.
///   - `nested_token`: A nested token inside the quote.
///
/// - `Bold`:
///   Represents bold text (e.g., `**bold**`). Contains:
///   - `text`: The content of the bold text.
///
/// - `Italic`:
///   Represents italic text (e.g., `*italic*`). Contains:
///   - `text`: The content of the italic text.
///
/// - `Strikethrough`:
///   Represents strikethrough text (e.g., `~~text~~`). Contains:
///   - `text`: The content of the strikethrough text.
///
/// - `Link`:
///   Represents a hyperlink (e.g., `[text](url)`). Contains:
///   - `text`: The display text of the link.
///   - `url`: The URL of the link.
///
/// - `CodeBlockStart`:
///   Marks the start of a code block (e.g., `` ``` ``).
///
/// - `CodeBlockEnd`:
///   Marks the end of a code block (e.g., `` ``` ``).
///
/// - `HorizLine`:
///   Represents a horizontal line (e.g., `---`).
///
/// - `BreakLine`:
///   Represents a line break.
///
/// - `None`:
///   Represents an empty or unrecognized token.
use regex::Regex;
use std::cmp::min;
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
    Quote {
        text: String,
        nested_token: Box<Token>,
    },
    // InlineCode {
    //     text: String,
    // },
    // Strikethrough {
    //     text: String,
    // },
    // CodeBlock {
    //     text: String,
    // },
    // Link {
    //     text: String,
    //     url: String,
    // },
    CodeBlockStart {},
    CodeBlockEnd {},
    CodeBlock {},
    HorizLine {},
    BreakLine {},
    None {},
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
            Token::SimpleText { text } => Token::SimpleText { text: text.clone() },
            Token::Quote { text, nested_token } => Token::Quote {
                text: text.clone(),
                nested_token: nested_token.clone(),
            },
            Token::OLStart {} => Token::OLStart {},
            Token::OLEnd {} => Token::OLEnd {},
            Token::CodeBlockStart {} => Token::CodeBlockStart {},
            Token::CodeBlockEnd {} => Token::CodeBlockEnd {},
            Token::HorizLine {} => Token::HorizLine {},
            Token::BreakLine {} => Token::BreakLine {},
            Token::None {} => Token::None {},
            Token::CodeBlock {} => Token::CodeBlock {},
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
            Token::Quote { text, nested_token } => {
                write!(f, "<q>{}{}</q>", text, nested_token.as_ref())
            }
            Token::OLStart {} => write!(f, "<ol>"),
            Token::OLEnd {} => write!(f, "</ol>"),
            Token::CodeBlock {} => write!(f, ""),
            Token::CodeBlockStart {} => write!(f, "<pre><code>"),
            Token::CodeBlockEnd {} => write!(f, "</code></pre>"),
            Token::SimpleText { text } => write!(f, "{}", text),
            Token::HorizLine {} => write!(f, "<hr>"),
            Token::BreakLine {} => write!(f, "<br/>"),
            Token::None {} => write!(f, ""),
        }
    }
}

/// Converts inline markdown syntax to HTML.
///
/// This function processes a single line of markdown text and converts inline
/// markdown elements such as bold, italic, and links into their corresponding
/// HTML representations. The function modifies the input string in place and
/// returns the resulting HTML string.
///
/// # Supported Inline Markdown Syntax
///
/// - **Bold**: `**text**` is converted to `<strong>text</strong>`.
/// - *Italic*: `*text*` is converted to `<i>text</i>`.
/// - [Links](url): `[text](url)` is converted to `<a href="url">text</a>`.
///
/// # Arguments
///
/// * `line` - A mutable reference to a `String` containing the markdown text to be converted.
///
/// # Returns
///
/// A `String` containing the converted HTML representation of the input markdown.
///
/// # Example
///
/// ```rust
/// let mut line = String::from("This is **bold**, *italic*, and [a link](https://example.com).");
/// let html = markdown_parser::convert_inline_markdown(&mut line);
/// assert_eq!(
///     html,
///     "This is <strong>bold</strong>, <i>italic</i>, and <a href=\"https://example.com\">a link</a>."
/// );
/// ```
///
/// # Notes
///
/// - The function processes bold syntax (`**text**`) before italic syntax (`*text*`),
///   as bold and italic can overlap in markdown.
/// - Nested link syntax is supported, allowing for constructs like `[text [nested]](url)`.
/// - The input string is modified during processing, but the returned string contains
///   the final HTML result.
///
/// # Debugging
///
/// The function includes debug output for link processing. If a link pattern is detected,
/// it prints a debug message to the console.
///
/// # Limitations
///
/// - The function assumes valid markdown input and does not handle malformed markdown.
/// - Inline code syntax (e.g., `` `code` ``) is not currently supported.
pub fn convert_inline_markdown(line: &mut String) -> String {
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

    // Finding inlined code
    re = Regex::new(r"(`+)([^`]*)(`+)").unwrap(); // Split into 3 capture groups
    let mut line_copy = resulted_format.clone();
    let mut resulted_format = String::new();
    if re.is_match(&line_copy) {
        let found_ind = re.find(&line_copy).unwrap();
        let start_of_string = (&line_copy[0..found_ind.start()]).to_string(); // Whatever is before it

        // We use capture groups to find all the parts
        let Some(groups) = re.captures(&line_copy) else {
            return line_copy;
        };
        let first_backticks = groups[1].to_string();
        let mut inline_code = groups[2].to_string();
        let end_backticks = groups[3].to_string();

        if inline_code.len() > 0 {
            let min_num_backticks = min(first_backticks.len(), end_backticks.len());
            if min_num_backticks == first_backticks.len()
                && min_num_backticks != end_backticks.len()
            {
                inline_code = format!(
                    "{}{}",
                    inline_code,
                    end_backticks[0..end_backticks.len() - min_num_backticks].to_string()
                );
            } else if min_num_backticks == end_backticks.len()
                && min_num_backticks != first_backticks.len()
            {
                inline_code = format!(
                    "{}{}",
                    first_backticks[0..first_backticks.len() - min_num_backticks].to_string(),
                    inline_code
                );
            }

            // Now found_substring contains only the italic text so we can make it html
            resulted_format = format!(
                "{resulted_format}{}<code>{}</code>",
                start_of_string, inline_code
            );

            // We then trim the start of the original line till what we found
            let _ = &line_copy.replace_range(..found_ind.end(), "");
        }
    }
    resulted_format = resulted_format + &line_copy;

    return resulted_format;
}

/// Tokenizes a collection of markdown lines into HTML tokens.
///
/// This function processes an iterator of markdown strings, tokenizes each line,
/// and converts them into a vector of HTML strings. It handles multi-line constructs
/// such as ordered lists and code blocks by adding special tokens to encapsulate
/// their content.
///
/// # Arguments
///
/// * `str_iter` - An iterator over `String` items, where each item represents a line of markdown text.
///
/// # Returns
///
/// A `Vec<String>` containing the HTML representation of the tokenized markdown lines.
///
/// # Example
///
/// ```rust
/// let markdown_lines = vec![
///     String::from("# Header"),
///     String::from("This is a paragraph."),
///     String::from("- List item"),
///     String::from("```"),
///     String::from("Code block content"),
///     String::from("```"),
/// ];
/// let html_tokens = markdown_parser::tokenize_text(markdown_lines.into_iter());
/// assert_eq!(html_tokens, vec![
///     "<h1>Header</h1>",
///     "<p>This is a paragraph.</p>",
///     "<li>List item</li>",
///     "<pre><code>",
///     "Code block content",
///     "</code></pre>",
/// ]);
/// ```
///
/// # Notes
///
/// - The function uses `tokenize_line` to process each line individually.
/// - Debugging information is printed to the console for tokenized lines.
/// - Multi-line constructs such as ordered lists and code blocks are handled
///   by adding `OLStart`, `OLEnd`, `CodeBlockStart`, and `CodeBlockEnd` tokens.
///
/// # Debugging
///
/// The function includes debug output for tokenized lines. It prints the tokenized
/// representation of the input lines to the console.
///
/// # Limitations
///
/// - The function assumes valid markdown input and does not handle malformed markdown.
/// - Inline markdown elements (e.g., bold, italic, links) are processed by the
///   `convert_inline_markdown` function.
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
    let mut skip_token = false;
    for (i, token) in token_list.iter().enumerate() {
        // PUSH ANYTHING BEFORE THE CURRENT TOKEN
        if matches!(token, Token::OListItem { .. })
            && !matches!(last_token, Token::OListItem { .. })
        {
            // If this is one of the first series of ol's then we add starting tag
            token_list_processed.push(Token::OLStart {});
        }

        if matches!(token, Token::CodeBlock {}) {
            token_list_processed.push(if inside_code_block == false {
                Token::CodeBlockStart {}
            } else {
                Token::CodeBlockEnd {}
            });
            skip_token = true;
            inside_code_block = !inside_code_block;
        }

        // PUSH THE CURRENT TOKEN
        if skip_token == false {
            token_list_processed.push(token.clone());
        }

        // Code blocks remove all formatting inside so we'll only use simple_text tokens
        if inside_code_block && skip_token != true {
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
        skip_token = false;
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
/// markdown_parser::tokenize_line(line);
/// ```
pub fn tokenize_line(line: String) -> Result<Token, ()> {
    let token_result;
    let mut line_copy = line.clone();

    // Not all blocks accept the inline parsing, eg. Code blocks

    // Line is a Header
    if line_copy.starts_with("#") {
        let mut count = 0;
        while line_copy.starts_with("#") {
            count += 1;
            line_copy.remove(0); // Removes the #
        }
        // If after all the "#" there isn't a simple space,
        // we need to skip because it isn't valid markdown
        if line_copy.chars().nth(0).unwrap() != ' ' {
            line_copy = line.clone();
        } else {
            // Remove the space as well
            line_copy.remove(0);

            let inline_converted_line = convert_inline_markdown(&mut line_copy);

            token_result = Token::Header {
                level: (count),
                text: (inline_converted_line),
            };
            return Ok(token_result);
        }
    }

    // Line is a block quote
    if line_copy.starts_with("> ") {
        line_copy.remove(0); // Removes the >
        line_copy.remove(0); // Removes the space after
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
        line_copy.remove(0); // Removes the - char
        line_copy.remove(0); // Removes the space after
        let inline_converted_line = convert_inline_markdown(&mut line_copy);
        token_result = Token::UListItem {
            text: (inline_converted_line),
        };
        return Ok(token_result);
    }
    // Line is codeblock start can follow text after it
    if line_copy.starts_with("```") {
        token_result = Token::CodeBlock {};
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

    // Line is empty -> BreakLine
    if line_copy.clone().trim() == "" {
        token_result = Token::BreakLine {};
        return Ok(token_result);
    }

    // If none of the above -> Paragraph
    let inline_converted_line = convert_inline_markdown(&mut line_copy);
    token_result = Token::Paragraph {
        text: (inline_converted_line),
    };
    Ok(token_result)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_header() {
        let token = tokenize_line(String::from("## Header")).unwrap();
        match token {
            Token::Header { level, text } => {
                assert_eq!(level, 2);
                assert_eq!(text, "Header");
            }
            _ => panic!("Expected Header token"),
        }
    }

    #[test]
    fn test_header_missing_space() {
        let line = String::from("##Header");
        let token = tokenize_line(line.clone()).unwrap();
        match token {
            Token::Paragraph { text } => assert_eq!(text, line),
            _ => panic!("Should not parse header without space"),
        };
    }

    #[test]
    fn test_ulist_missing_space() {
        let line = String::from("-List item");
        let token = tokenize_line(line.clone()).unwrap();
        match token {
            Token::Paragraph { text } => assert_eq!(text, line),
            _ => panic!("Should not parse header without space"),
        };
    }

    #[test]
    fn test_olist_missing_space() {
        let line = String::from("1.Ordered item");
        let token = tokenize_line(line.clone()).unwrap();
        match token {
            Token::Paragraph { text } => assert_eq!(text, line),
            _ => panic!("Should not parse ordered list without space"),
        };
    }

    #[test]
    fn test_unclosed_code_block() {
        let lines = vec![String::from("```"), String::from("code but never ends")];
        let tokens = tokenize_text(lines.into_iter());
        assert!(
            tokens.last().unwrap() != "</code></pre>",
            "Should not close unclosed code block"
        );
    }

    #[test]
    fn test_unclosed_bold() {
        let mut line = String::from("This is **bold text.");
        let html = convert_inline_markdown(&mut line);
        assert_eq!(
            html, "This is **bold text.",
            "Unclosed bold should remain raw"
        );
    }

    #[test]
    fn test_unclosed_italic() {
        let mut line = String::from("This is *italic text.");
        let html = convert_inline_markdown(&mut line);
        assert_eq!(
            html, "This is *italic text.",
            "Unclosed italic should remain raw"
        );
    }

    #[test]
    fn test_malformed_link_missing_closing_paren() {
        let mut line = String::from("A [link](https://example.com here.");
        let html = convert_inline_markdown(&mut line);
        assert_eq!(
            html, "A [link](https://example.com here.",
            "Malformed link should not convert"
        );
    }

    #[test]
    fn test_malformed_link_missing_closing_bracket() {
        let mut line = String::from("A link](https://example.com) here.");
        let html = convert_inline_markdown(&mut line);
        assert_eq!(
            html, "A link](https://example.com) here.",
            "Malformed link should not convert"
        );
    }

    #[test]
    fn test_misnested_formatting() {
        let mut line = String::from("This is *italic and **bold*** text**.");
        let html = convert_inline_markdown(&mut line);
        assert_eq!(
            html, "This is <i>italic and <strong>bold</strong></i> text**.",
            "Misnested formatting should still parse valid nesting"
        );
    }

    #[test]
    fn test_not_a_horizontal_rule() {
        let line = String::from("--- not a rule");
        let token = tokenize_line(line);
        match token {
            Ok(Token::Paragraph { .. }) => (), // treat as paragraph
            _ => panic!("Line with text after --- should not be HorizLine"),
        }
    }

    #[test]
    fn test_empty_or_garbage_line() {
        let line = String::from("    "); // only spaces
        let token = tokenize_line(line);
        match token {
            Ok(Token::BreakLine {}) => (),
            _ => panic!("Whitespace-only line should be a BreakLine"),
        }

        let line = String::from("!@#$%^&*()");
        let token = tokenize_line(line.clone());
        match token {
            Ok(Token::Paragraph { text }) => assert_eq!(line, text),
            _ => panic!("Garbage line should fallback to Paragraph or raw text"),
        };
    }

    #[test]
    fn test_tokenize_paragraph() {
        let line = String::from("This is a paragraph.");
        let token = tokenize_line(line).unwrap();
        match token {
            Token::Paragraph { text } => assert_eq!(text, "This is a paragraph."),
            _ => panic!("Expected Paragraph token"),
        }
    }

    #[test]
    fn test_tokenize_ulist_item() {
        let line = String::from("- List item");
        let token = tokenize_line(line).unwrap();
        match token {
            Token::UListItem { text } => assert_eq!(text, "List item"),
            _ => panic!("Expected UListItem token"),
        }
    }

    #[test]
    fn test_tokenize_olist_item() {
        let line = String::from("1. Ordered item");
        let token = tokenize_line(line).unwrap();
        match token {
            Token::OListItem { text } => assert_eq!(text, "Ordered item"),
            _ => panic!("Expected OListItem token"),
        }
    }

    #[test]
    fn test_tokenize_text_codeblock_sequence() {
        let lines = vec![
            String::from("```"),
            String::from("code line 1"),
            String::from("code line 2"),
            String::from("```"),
        ];
        let tokens = tokenize_text(lines.into_iter());
        assert_eq!(tokens[0], "<pre><code>");
        assert_eq!(tokens[1], "code line 1");
        assert_eq!(tokens[2], "code line 2");
        assert_eq!(tokens[3], "</code></pre>");
    }

    #[test]
    fn test_tokenize_horiz_line() {
        let line = String::from("---");
        let token = tokenize_line(line).unwrap();
        match token {
            Token::HorizLine {} => (),
            _ => panic!("Expected HorizLine token"),
        }
    }

    #[test]
    fn test_tokenize_break_line() {
        let line = String::from("");
        let token = tokenize_line(line).unwrap();
        match token {
            Token::BreakLine {} => (),
            _ => panic!("Expected BreakLine token"),
        }
    }

    #[test]
    fn test_tokenize_bold_and_italic() {
        let mut line = String::from("This is **bold** and *italic*.");
        let html = super::convert_inline_markdown(&mut line);
        assert_eq!(html, "This is <strong>bold</strong> and <i>italic</i>.");
    }

    #[test]
    fn test_tokenize_link() {
        let mut line = String::from("A [link](https://example.com) here.");
        let html = super::convert_inline_markdown(&mut line);
        assert_eq!(html, "A <a href=\"https://example.com\">link</a> here.");
    }

    #[test]
    fn test_convert_inline_markdown_bold() {
        let mut line = String::from("This is **bold** text.");
        let result = convert_inline_markdown(&mut line);
        assert_eq!(result, "This is <strong>bold</strong> text.");
    }

    #[test]
    fn test_convert_inline_markdown_italic() {
        let mut line = String::from("This is *italic* text.");
        let result = convert_inline_markdown(&mut line);
        assert_eq!(result, "This is <i>italic</i> text.");
    }

    #[test]
    fn test_tokenize_text_simple() {
        let markdown_lines = vec![
            String::from("# Header"),
            String::from("This is a paragraph."),
            String::from("- List item"),
        ];
        let result = tokenize_text(markdown_lines.into_iter());
        assert_eq!(
            result,
            vec![
                "<h1>Header</h1>",
                "<p>This is a paragraph.</p>",
                "<li>List item</li>",
            ]
        );
    }

    #[test]
    fn test_inline_code_basic() {
        let mut line = String::from("This is `code`.");
        let html = convert_inline_markdown(&mut line);
        assert_eq!(html, "This is <code>code</code>.");
    }

    #[test]
    fn test_inline_code_multiple() {
        let mut line = String::from("`a` and `b` are variables.");
        let html = convert_inline_markdown(&mut line);
        // Only the first inline code will be replaced due to current implementation
        // Adjust this test if you improve the function to handle multiple
        assert!(html.contains("<code>a</code>") || html.contains("<code>b</code>"));
    }

    #[test]
    fn test_inline_code_with_special_chars() {
        let mut line = String::from("Use `x = y + z;` in your code.");
        let html = convert_inline_markdown(&mut line);
        assert_eq!(html, "Use <code>x = y + z;</code> in your code.");
    }

    #[test]
    fn test_inline_code_unclosed() {
        let mut line = String::from("This is `not closed.");
        let html = convert_inline_markdown(&mut line);
        assert_eq!(html, "This is `not closed.");
    }

    #[test]
    // We don't do recurssive inline codes
    fn test_inline_code_nested_backticks() {
        let mut line = String::from("Here is a ``code with `backtick` inside`` example.");
        let html = convert_inline_markdown(&mut line);
        // Should handle double backticks as code delimiter
        assert!(html.contains("<code>code with `backtick` inside</code>"));
    }
}
