use std::fs;
use syn::{visit::Visit, File, ItemStruct};

struct StructNameVisitor {
    pub names: Vec<String>,
}

impl<'ast> Visit<'ast> for StructNameVisitor {
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        self.names.push(node.ident.to_string());
        syn::visit::visit_item_struct(self, node);
    }
}

fn main() {
    println!("WE ARE HEREEEEEE");
    let filename = "qoqo/src/operations/single_qubit_gate_operations.rs";
    let content = fs::read_to_string(filename).expect("Could not read file");
    let syntax_tree: File = syn::parse_file(&content).expect("Failed to parse file");

    let mut visitor = StructNameVisitor { names: Vec::new() };
    visitor.visit_file(&syntax_tree);

    println!("WE ARE HERE");
    for name in &visitor.names {
        println!("{}", name);
    }
}
