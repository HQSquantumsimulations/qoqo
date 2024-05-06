use std::{fs, path::PathBuf};

use pyo3::{
    types::{PyAnyMethods, PyDict, PyModule},
    Python,
};
use regex::Regex;

fn str_to_type(res: &str) -> Option<String> {
    match res {
        s if s.contains("Pragma") => Some("Operation".to_owned()),
        "CalculatorFloat" => Some("Tuple[float, str]".to_owned()),
        "String" | "string" => Some("str".to_owned()),
        "" => None,
        "uint" => Some("int".to_owned()),
        "self" => Some("Circuit".to_owned()),
        _ => Some(
            res.replace("list", "List")
                .replace("dict", "Dict")
                .replace("tuple", "Tuple")
                .replace("set", "Set")
                .replace("circuit", "Circuit")
                .replace("optional", "Optional")
                .replace("operation", "Operation")
                .to_owned(),
        ),
    }
}

fn extract_type(string: &str) -> Option<String> {
    let pattern = r"\(([a-zA-Z_\[\] ,|]+?)\)";
    let re = Regex::new(pattern).unwrap();
    if let Some(captures) = re.captures(string) {
        if let Some(res) = captures.get(1).map(|s| s.as_str()) {
            str_to_type(res)
        } else {
            None
        }
    } else {
        None
    }
}

fn collect_args_from_doc(doc: &str) -> Vec<String> {
    let args_vec: Vec<_> = doc
        .split("\n")
        .skip_while(|&line| line != "Args:")
        .skip(1)
        .skip_while(|line| line.len() == 0)
        .take_while(|line| line.len() != 0)
        .collect();
    args_vec
        .iter()
        .filter_map(|&line| {
            (line.contains(':') && line.trim().starts_with(char::is_alphabetic)).then(|| {
                format!(
                    "{}{}",
                    line.trim().split_once([' ', ':']).unwrap_or(("", "")).0,
                    extract_type(line)
                        .map(|arg_type| format!(": {}", arg_type))
                        .unwrap_or_default()
                )
            })
        })
        .collect()
}

fn collect_return_from_doc(doc: &str) -> String {
    let args_vec: Vec<_> = doc
        .split("\n")
        .skip_while(|&line| line != "Returns:")
        .skip(1)
        .take(1)
        .filter(|&line| line.contains(':') && line.trim().starts_with(char::is_alphabetic))
        .collect();
    if args_vec.len() == 0 {
        "".to_owned()
    } else if let Some(ret) =
        str_to_type(args_vec[0].trim().split_once([':']).unwrap_or(("", "")).0)
    {
        format!(" -> {}", ret)
    } else {
        "".to_owned()
    }
}

fn create_doc(module: &str) -> String {
    let mut module_doc = if module == "qoqo" {
        "from typing import Optional, List, Tuple, Dict, Set\n\n".to_owned()
    } else {
        "from qoqo import Circuit, Operation\nfrom typing import Tuple, List, Optional, Dict\n\n"
            .to_owned()
    };
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let python_module = PyModule::import_bound(py, module).unwrap();
        let dict = python_module.as_gil_ref().getattr("__dict__").unwrap();
        let r_dict = dict.downcast::<PyDict>().unwrap();
        for (fn_name, func) in r_dict.iter() {
            let name = fn_name.str().unwrap().extract::<String>().unwrap();
            if name.starts_with("__")
                || (module == "qoqo"
                    && ![
                        "qoqo",
                        "Circuit",
                        "QuantumProgram",
                        "CircuitDag",
                        "operations",
                    ]
                    .contains(&name.as_str()))
            {
                continue;
            }
            let doc = func
                .getattr("__doc__")
                .unwrap()
                .extract::<String>()
                .unwrap();
            if name != "qoqo" {
                let args = collect_args_from_doc(doc.as_str()).join(", ");
                module_doc.push_str(&format!(
                    "class {name}{}:\n    \"\"\"\n{doc}\n\"\"\"\n\n    def __init__(self{}):\n       return\n\n",
                    module.eq("operations").then_some("(Operation)").unwrap_or_default(),
                    args.is_empty().then_some("").unwrap_or(format!(", {}", args).as_str()),
                ));
                let class_dict = func.getattr("__dict__").unwrap();
                let items = class_dict.call_method0("items").unwrap();
                let dict_obj = py
                    .import_bound("builtins")
                    .unwrap()
                    .call_method1("dict", (items,))
                    .unwrap();
                let class_r_dict = dict_obj.as_gil_ref().downcast::<PyDict>().unwrap();
                for (class_fn_name, meth) in class_r_dict.iter() {
                    let meth_name = class_fn_name.str().unwrap().extract::<String>().unwrap();
                    let class_doc = match meth_name.as_str() {
                        "__add__" => r#"Implement the `+` (__add__) magic method to add two Circuits.

Args:
    rhs (Operation | Circuit): The second Circuit object in this operation.

Returns:
    Circuit: self + rhs the two Circuits added together.

    Raises:
    TypeError: Left hand side can not be converted to Circuit.
    TypeError: Right hand side cannot be converted to Operation or Circuit."#.to_owned(),
                        "__iadd__" => r#"Implement the `+=` (__iadd__) magic method to add a Operation to a Circuit.

Args:
    other (Operation | Circuit): The Operation object to be added to self.

Returns:
    Circuit: self + other the two Circuits added together as the first one.

Raises:
    TypeError: Right hand side cannot be converted to Operation or Circuit."#.to_owned(),
                        "__new__" => "".to_owned(),
                        _ => meth
                            .getattr("__doc__")
                            .unwrap()
                            .extract::<String>()
                            .unwrap_or_default(),
                    };
                    if class_doc.eq("") {
                        continue;
                    }
                    let meth_args = collect_args_from_doc(class_doc.as_str()).join(", ");
                    module_doc.push_str(&format!(
                        "    @classmethod\n    def {meth_name}(self{}){}: # type: ignore\n        \"\"\"\n{class_doc}\n\"\"\"\n\n",
                        meth_args.is_empty().then_some("").unwrap_or(format!(", {}", meth_args).as_str()),
                        collect_return_from_doc(class_doc.as_str())
                    ));
                }
            } else if name == "operations" {
                module_doc.push_str(&format!(
                    "class Operation:\n    \"\"\"\n{doc}\n\"\"\"\n\n    def __init__(self):\n       return\n\n",
                ));
            } else {
                module_doc.push_str(&format!(
                    "def {name}({}){}:\n    \"\"\"\n{doc}\n\"\"\"\n\n",
                    collect_args_from_doc(doc.as_str()).join(", "),
                    module
                        .eq("qoqo.operations")
                        .then(|| " -> Operation")
                        .unwrap_or_default()
                ));
            }
        }
    });
    module_doc
}

#[test]
fn test_doc() {
    for &module in [
        "qoqo",
        "operations",
        "measurements",
        "noise_models",
        "devices",
    ]
    .iter()
    {
        let qoqo_doc = create_doc(
            module
                .eq("qoqo")
                .then_some(module)
                .unwrap_or(&format!("qoqo.{module}")),
        );
        let out_dir = PathBuf::from(format!("qoqo/{}.pyi", module));
        fs::write(&out_dir, qoqo_doc).expect("Could not write to file");
    }
    assert!(false);
}
