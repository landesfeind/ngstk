use svg::internal::*;

mod internal;
mod text;
mod dna;


pub fn sketch<T: SvgDecorator>(content: &T, width: Option<f64>, height: Option<f64>) -> String {
    let doc = Document::new();
    let svg = doc.create_element(ElementId::Svg);
    doc.append(&svg);

    let content_node = content.to_svg(&doc, &Scale::new(), &Scale::new());
    svg.append(&content_node);

    if width.is_some() {
        svg.set_attribute(AttributeId::Width, width.unwrap());
    }
    if height.is_some() {
        svg.set_attribute(AttributeId::Height, height.unwrap());
    }
    
    doc.to_string_with_opt(&WriteOptions::default())
}
