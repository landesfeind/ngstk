use svg::internal::*;

impl SvgDecorator for String {

    fn width(&self) -> f64 {
        (self.len() * 10) as f64
    }

    fn height(&self) -> f64 {
        15f64
    }

    fn to_svg(&self, doc: &Document, xscale: &Scale, yscale: &Scale) -> Node {
        let self_node = doc.create_element(ElementId::Text);
        let textnode = doc.create_node(NodeType::Text, &self.as_ref());
        self_node.append(&textnode);

        self_node.set_attribute(AttributeId::X, xscale.to(self.width() / 2.0));
        self_node.set_attribute(AttributeId::Y, yscale.to(self.height() - 2.5));

        self_node.set_attribute(AttributeId::FontSize, "12px");
        self_node.set_attribute(AttributeId::TextAnchor, "middle");
        self_node.set_attribute(AttributeId::DominantBaseline, "middle");

        self_node
    }

}

