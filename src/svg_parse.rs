use std::{ fs, error::Error };
use xmltree::{ XMLNode, Element };

type NodeBox = Box<XMLNode>;
type NodeVector = Vec<NodeBox>;

#[derive(Clone)]
pub struct ParsedSVG {
    element: Element,
    text_elements: Option<NodeVector>,
    image_elements: Option<NodeVector>
}

pub fn parse_xml_file(file_name: &str) -> Result<ParsedSVG, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_name)?;
    let root_element = Element::parse(file_content.as_bytes())?;

    Ok(
        ParsedSVG::new(root_element)
    )
}

impl ParsedSVG {
    pub fn new(element: Element) -> Self {
        let mut element = Self {
            element: element,
            text_elements: None,
            image_elements: None
        };

        element.text_elements = Some(element.get_elements_of_name("text"));
        element.image_elements = Some(element.get_elements_of_name("image"));

        element
    }

    fn get_elements_of_name(&mut self, name: &'static str) -> NodeVector {
        let mut nodes = Vec::<NodeBox>::new();
        
        for element in &self.element.children {
            let node = element.as_element();
            if node == None { continue };

            let node = node.unwrap();

            if node.name == name {
                nodes.push(Box::new(element.to_owned()));
            }
            else {
                let mut node = ParsedSVG::new(node.to_owned());
                nodes.append(&mut node.get_elements_of_name(name));
            }
        }

        nodes
    }
}