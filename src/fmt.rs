use dissimilar::Chunk;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, HeadingLevel, Tag};
use text_edit::{TextEdit, TextRange, TextSize};

#[allow(clippy::too_many_lines)]
pub(crate) fn fmt(text: &str) -> TextEdit {
    let mut output = String::new();

    let mut parser = pulldown_cmark::Parser::new(text).peekable();

    let mut in_paragraph = false;
    let mut paragraph_text = String::new();
    let mut list_num = None;

    let mut nested_level = 0;
    let mut is_item_start = false;

    while let Some(event) = parser.next() {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    assert!(!in_paragraph);
                    in_paragraph = true;
                    output.push('\n');
                }
                Tag::Heading(level, _identifier, _classes) => {
                    assert!(!in_paragraph);
                    let tag = match level {
                        HeadingLevel::H1 => "#",
                        HeadingLevel::H2 => "##",
                        HeadingLevel::H3 => "###",
                        HeadingLevel::H4 => "####",
                        HeadingLevel::H5 => "#####",
                        HeadingLevel::H6 => "######",
                    };
                    output.push('\n');
                    output.push_str(tag);
                    output.push(' ');
                }
                Tag::BlockQuote => todo!(),
                Tag::CodeBlock(kind) => {
                    assert!(!in_paragraph);
                    let language_identifier = match kind {
                        CodeBlockKind::Indented => CowStr::Borrowed(""),
                        CodeBlockKind::Fenced(ident) => ident,
                    };
                    output.push_str("\n```");
                    output.push_str(&language_identifier);
                    output.push('\n');
                }
                Tag::List(starting_num) => {
                    println!("entering list");
                    tracing::error!("hello");
                    nested_level += 1;
                    list_num = starting_num;
                    output.push('\n');
                }
                Tag::Item => {
                    match list_num {
                        Some(num) => {
                            println!("entering item");
                            output.push_str(&format!("{num}. "));
                            list_num = list_num.map(|num| num + 1);
                        }
                        None => {
                            println!("entering item");
                            output.push_str("- ");
                        }
                    }
                    is_item_start = true;
                }
                Tag::FootnoteDefinition(_def) => todo!(),
                Tag::Table(_table) => todo!(),
                Tag::TableHead => todo!(),
                Tag::TableRow => todo!(),
                Tag::TableCell => todo!(),
                Tag::Emphasis => {
                    assert!(in_paragraph);
                    paragraph_text.push('*');
                }
                Tag::Strong => {
                    assert!(in_paragraph);
                    paragraph_text.push_str("**");
                }
                Tag::Strikethrough => todo!(),
                Tag::Link(_ty, _destination, _title) => todo!(),
                Tag::Image(_ty, _destination, _title) => todo!(),
            },
            Event::End(tag) => {
                match tag {
                    Tag::Paragraph => {
                        assert!(in_paragraph);
                        in_paragraph = false;

                        let subsequent_indent = "  ".repeat(nested_level);
                        let mut options = textwrap::Options::new(80);
                        options.subsequent_indent = &subsequent_indent;
                        let wrapped_text = textwrap::fill(&paragraph_text, options);

                        output.push_str(&wrapped_text);
                        output.push('\n');
                        // if parser.peek().is_some() {
                        //     output.push('\n');
                        // }

                        paragraph_text = String::new();
                    }
                    Tag::Heading(_level, _identifier, _classes) => {
                        assert!(!in_paragraph);
                        output.push('\n');
                    }
                    Tag::BlockQuote => todo!(),
                    Tag::CodeBlock(_kind) => {
                        assert!(!in_paragraph);
                        output.push_str("```\n");
                    }
                    Tag::List(_) => {
                        // TODO
                        list_num = None;
                        nested_level -= 1;
                        println!("exiting list: {nested_level}");
                        if nested_level == 0 {
                            output.push('\n');
                        }
                    }
                    Tag::Item => {
                        println!("exiting item");
                        match parser.peek() {
                            Some(Event::End(Tag::List(_))) => {}
                            _ => output.push('\n'),
                        }
                    }
                    Tag::FootnoteDefinition(_def) => todo!(),
                    Tag::Table(_table) => todo!(),
                    Tag::TableHead => todo!(),
                    Tag::TableRow => todo!(),
                    Tag::TableCell => todo!(),
                    Tag::Emphasis => {
                        assert!(in_paragraph);
                        paragraph_text.push('*');
                    }
                    Tag::Strong => {
                        assert!(in_paragraph);
                        paragraph_text.push_str("**");
                    }
                    Tag::Strikethrough => todo!(),
                    Tag::Link(_ty, _destination, _title) => todo!(),
                    Tag::Image(_ty, _destination, _title) => todo!(),
                };
                is_item_start = false;
            }
            Event::Text(text) => {
                if in_paragraph {
                    println!("pushing paragraph: {text:?}");
                    paragraph_text.push_str(&text);
                } else {
                    println!("pushing output: {text:?}");
                    output.push_str(&text);
                }
                is_item_start = false;
            }
            Event::Code(code) => {
                assert!(in_paragraph);
                paragraph_text.push_str(&format!("`{code}`"));
                is_item_start = false;
            }
            Event::Html(_html) => todo!(),
            Event::FootnoteReference(_fr) => todo!(),
            Event::SoftBreak => {
                if in_paragraph {
                    paragraph_text.push(' ');
                }
            }
            Event::HardBreak => todo!(),
            Event::Rule => todo!(),
            Event::TaskListMarker(_checked) => todo!(),
        }
        // println!("{}", output.trim_start());
    }

    panic!("{}", output.trim_start());

    // diff(text, &output)
}

#[allow(unused)]
pub(crate) fn diff(left: &str, right: &str) -> TextEdit {
    let chunks = dissimilar::diff(left, right);
    let mut builder = TextEdit::builder();
    let mut pos = TextSize::default();

    let mut chunks = chunks.into_iter().peekable();
    while let Some(chunk) = chunks.next() {
        if let (Chunk::Delete(deleted), Some(&Chunk::Insert(inserted))) = (chunk, chunks.peek()) {
            chunks.next().unwrap();
            let deleted_len = TextSize::of(deleted);
            builder.replace(TextRange::at(pos, deleted_len), inserted.into());
            pos += deleted_len;
            continue;
        }

        match chunk {
            Chunk::Equal(text) => {
                pos += TextSize::of(text);
            }
            Chunk::Delete(deleted) => {
                let deleted_len = TextSize::of(deleted);
                builder.delete(TextRange::at(pos, deleted_len));
                pos += deleted_len;
            }
            Chunk::Insert(inserted) => {
                builder.insert(pos, inserted.into());
            }
        }
    }
    builder.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_fmt() {
        let md = "
- hello creauh arlochu reclaoh urclao huroaelhu rclaoehu rcleoah urlcoaeh urlcoae hurloeah urlcoea \
                  hurlcaoeh ulrcaoe hulrcoea hulrcaoe hurlcoe aeocluhaoelrcuh oareclu hrloacu \
                  hrocaleu hroalcu haorcel u

  rcoeahu alrcohu lraceouh lrcaeohulrcaohu lrcaoehu lrcaeohurlcaoe rlucoah elcruh oaelcruh aorlch \
                  ulrcoeah ucrl
- hello
- goodbye

she sells seashells
by the seashore
";
        panic!("{:?}", fmt(md));
    }
}
