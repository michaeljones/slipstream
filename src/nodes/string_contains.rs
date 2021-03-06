
use yaml_rust::Yaml;

use std::rc::Rc;
use std::cell::RefCell;

use Spec;
use SpecAttribute;
use Node;
use NodeRef;
use NodeBuilder;
use NodeUI;
use NodeUIData;
use StringFieldData;
use FlowData;

pub struct StringContains {
    pub id: i64,
    pub input: Option<Rc<RefCell<Node>>>,
    pub value: String,
}

impl Node for StringContains {
    fn id(&self) -> i64 {
        self.id
    }
    fn pull(&mut self) -> FlowData {
        match self.input {
            None => return FlowData::Error("No input".to_string()),
            Some(ref mut input) => {
                let content = input.borrow_mut().pull();

                return match content {
                    FlowData::StringArray(lines) => {
                        let mut output = vec![];
                        for i in &lines {
                            if i.contains(self.value.as_str()) {
                                output.push(i.to_string());
                            }
                        }
                        return FlowData::StringArray(output);
                    }
                    FlowData::Error(string) => FlowData::Error(string),
                    _ => FlowData::Error("Unknown data".to_string()),
                };
            }
        }
    }

    fn set_input(&mut self, node: Option<Rc<RefCell<Node>>>, _index: Option<i64>) -> () {
        self.input = node;
    }

    fn get_ui(&self) -> NodeUI {
        NodeUI::StringField(StringFieldData {
            label: String::from("Value"),
            field: String::from("value"),
        })
    }

    fn get_value(&self, field: &String) -> NodeUIData {
        if field == "value" {
            return NodeUIData::StringData(self.value.clone());
        }
        NodeUIData::None
    }

    fn set_value(&mut self, field: &String, data: NodeUIData) {
        match (field.as_ref(), data) {
            ("value", NodeUIData::StringData(string)) => {
                self.value = string;
            }
            _ => {}
        }
    }

    fn get_spec(&self) -> Spec {
        Spec {
            id: self.id,
            type_: String::from("string-contains"),
            attributes: vec![
                SpecAttribute::String(String::from("value"), self.value.clone()),
            ],
        }
    }
}

pub struct StringContainsBuilder {}

impl NodeBuilder for StringContainsBuilder {
    fn build(&self, id: i64, name: &str, entry: &Yaml) -> Option<NodeRef> {
        if name == "lines" {
            if let Some(value) = entry["value"].as_str() {
                return Some(Rc::new(RefCell::new(StringContains {
                    id: id,
                    input: None,
                    value: String::from(value),
                })));
            } else {
                println!("No 'value' for string contains node");
            }
        }
        None
    }
}
