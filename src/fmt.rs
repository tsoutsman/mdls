use dissimilar::Chunk;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Tag};
use text_edit::{TextEdit, TextRange, TextSize};

#[allow(clippy::too_many_lines)]
pub(crate) fn fmt(text: &str) -> TextEdit {
    let mut output = String::new();

    let mut parser = pulldown_cmark::Parser::new(text).peekable();

    let mut in_paragraph = false;
    let mut paragraph_text = String::new();

    while let Some(event) = parser.next() {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    assert!(!in_paragraph);
                    in_paragraph = true;
                }
                Tag::Heading(_level, _identifier, _classes) => todo!(),
                Tag::BlockQuote => todo!(),
                Tag::CodeBlock(kind) => {
                    let language_identifier = match kind {
                        CodeBlockKind::Indented => CowStr::Borrowed(""),
                        CodeBlockKind::Fenced(ident) => ident,
                    };
                    assert!(!in_paragraph);
                    output.push_str("```");
                    output.push_str(&language_identifier);
                    output.push('\n');
                }
                Tag::List(_starting_num) => todo!(),
                Tag::Item => todo!(),
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
            Event::End(tag) => match tag {
                Tag::Paragraph => {
                    assert!(in_paragraph);
                    in_paragraph = false;

                    let options = textwrap::Options::new(80);
                    let wrapped_text = textwrap::fill(&paragraph_text, options);

                    output.push_str(&wrapped_text);
                    output.push('\n');
                    if parser.peek().is_some() {
                        output.push('\n');
                    }

                    paragraph_text = String::new();
                }
                Tag::Heading(_level, _identifier, _classes) => todo!(),
                Tag::BlockQuote => todo!(),
                Tag::CodeBlock(_kind) => {
                    assert!(!in_paragraph);
                    output.push_str("```\n\n");
                }
                Tag::List(_starting_num) => todo!(),
                Tag::Item => todo!(),
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
            Event::Text(text) => {
                if in_paragraph {
                    paragraph_text.push_str(&text);
                } else {
                    output.push_str(&text);
                }
            }
            Event::Code(code) => {
                assert!(in_paragraph);
                paragraph_text.push_str(&format!("`{code}`"));
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
    }

    diff(text, &output)
}

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
