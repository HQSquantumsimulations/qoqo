// syn = { version = "2.0", features = ["full", "visit"] }

use std::{env, fs, path::PathBuf};
use syn::{visit::Visit, File, ItemStruct};

struct StructFirstDocLineVisitor {
    pub names: Vec<String>,
    pub descriptions: Vec<String>,
}

impl<'ast> Visit<'ast> for StructFirstDocLineVisitor {
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        self.names.push(node.ident.to_string());
        for attr in &node.attrs {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(meta_name_value) = &attr.meta {
                    if let syn::Expr::Lit(str) = &meta_name_value.value {
                        if let syn::Lit::Str(doc_str) = &str.lit {
                            let first_line = doc_str.value();
                            self.descriptions.push(first_line);
                            break;
                        }
                    }
                }
            }
        }
        syn::visit::visit_item_struct(self, node);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // e.g. if we run "cargo single run build_gates_list.rs -- filename.rs" then this will be "filename.rs"
    let filename_arg = &args[2];
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push(format!("roqoqo/src/operations/{}", filename_arg));
    let filename = path.to_str().unwrap();

    let content = fs::read_to_string(filename).expect("Could not read file");
    let syntax_tree: File = syn::parse_file(&content).expect("Failed to parse file");
    let mut visitor = StructFirstDocLineVisitor {
        names: Vec::new(),
        descriptions: Vec::new(),
    };
    visitor.visit_file(&syntax_tree);

    let mut combined: Vec<(String, String)> = visitor
        .names
        .iter()
        .cloned()
        .zip(visitor.descriptions.iter().cloned())
        .collect();
    combined.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));
    for (name, descr) in combined.iter() {
        println!("| {}     |{}  |", name, descr);
    }
}
