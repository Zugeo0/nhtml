use crate::parser::{Element, Tag};

pub fn emit_html(elements: Vec<Element>) -> String {
    let mut html = String::new();

    for element in &elements {
        emit_element(&mut html, element, 0)
    }

    html
}

fn emit_element(html: &mut String, element: &Element, indent: usize) {
    match element {
        Element::Tag(tag) => emit_tag(html, tag, indent),
        Element::Text(text) => html.push_str(&format!("{}{}\n", indent_str(indent), text)),
        Element::HTML(raw_html) => html.push_str(&format!("{}{}\n", indent_str(indent), raw_html)),
    }
}

fn emit_tag(html: &mut String, tag: &Tag, indent: usize) {
    html.push_str(&format!("{}<{}", indent_str(indent), tag.ty));

    for (name, value) in &tag.attribs {
        emit_attrib(html, name.clone(), value.clone());
    }

    let inline = tag.ty == "meta" || tag.ty == "link";

    html.push_str(">");

    if inline || tag.body.len() > 0 {
        html.push('\n');
    }

    for element in &tag.body {
        emit_element(html, element, indent + 1);
    }

    if tag.body.len() > 0 {
        html.push_str(&indent_str(indent));
    }

    if !inline {
        html.push_str(&format!("</{}>\n", tag.ty));
    }
}

fn emit_attrib(html: &mut String, name: String, value: Option<String>) {
    html.push_str(&format!(" {name}"));

    if let Some(val) = value {
        html.push_str(&format!("={val}"));
    }
}

fn indent_str(indent: usize) -> String {
    "    ".repeat(indent)
}
