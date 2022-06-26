macro_rules! this_init_node {
    ($this:expr,$classes: expr, $options:expr, $style:expr) => {
        $this.classes = $classes;
        $this.attributes = HashMap::new();
        $this.height = 0.0;
        $this.depth = 0.0;
        $this.max_font_size = 0.0;
        $this.style = $style;
        if let Some(opt) = $options {
            if opt.style().isTight() {
                $this.classes.push("mtight".to_string());
            }
            let c = opt.getColor();
            $this.style.color = Some(c);
        }
    };
}
pub(crate) use this_init_node;

macro_rules! this_to_node {
    ($this:expr, $tag_name:literal) => {{
        let document = web_sys::window().expect("").document().expect("");
        let node = document.create_element($tag_name).expect("");
        // Apply the class
        web_sys::Element::set_attribute(&node, "className", $this.classes.join(" ").as_str());

        // Apply inline styles
        let css_str = $this.style.to_css_str();
        if css_str != "" {
            web_sys::Element::set_attribute(&node, "style", css_str.as_str());
        }
        // Apply attributes
        for (k, v) in $this.attributes.iter() {
            web_sys::Element::set_attribute(&node, k.as_str(), v.as_str());
        }

        // Append the children, also as HTML nodes
        for child in $this.children.iter() {
            node.append_child(&child.to_node());
        }
        web_sys::Node::from(node)
    }};
}
pub(crate) use this_to_node;

macro_rules! this_to_markup {
    ($this:expr, $tag_name:literal) => {{
        let mut markup = format!("<{}", $tag_name);
        // Add the class

        if $this.classes.len() > 0 {
            let cl = $this.classes.join(" ");

            markup.push_str(&format!(" class=\"{}\"", escape(&cl)));
        }

        let styles = $this.style.to_css_str();

        if styles != "" {
            markup.push_str(&format!(" style=\"{}", escape(&styles)));
        }

        // Add the attributes
        for (k, v) in $this.attributes.iter() {
            markup.push_str(&format!(" {}={}", k, escape(&v)));
        }

        markup.push_str(">");

        // Add the markup of the children, also as markup
        for child in $this.children.iter() {
            markup.push_str(&child.to_markup());
        }

        markup.push_str(&format!("</{}>", $tag_name));

        markup
    }};
}

pub(crate) use this_to_markup;
