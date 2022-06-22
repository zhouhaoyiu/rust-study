fn main() {
    println!("Hello, world!");
    let test_js_code = "let x = 5;console.log(x)";
    init(test_js_code);
}
fn init(js_code: &str) {
    let _jskeyword = [
        "Break",
        "Case",
        "Catch",
        "Class",
        "Const",
        "Continue",
        "Debugger",
        "Default",
        "Delete",
        "Do",
        "Else",
        "Export",
        "Extends",
        "False",
        "Finally",
        "For",
        "Function",
        "If",
        "Import",
        "In",
        "Instanceof",
        "Let",
        "New",
        "Null",
        "Return",
        "Super",
        "Switch",
        "This",
        "Throw",
        "True",
        "Try",
        "Typeof",
        "Var",
        "Void",
        "While",
        "With",
        "Yield",
    ];
    let js_code_vec = js_code.split(";").collect::<Vec<&str>>();
    println!("{:?}", js_code_vec);
    for i in js_code_vec {
        for j in _jskeyword.iter() {
            // 将j转化为小写
            let j_lower = j.to_lowercase();
            if i.contains(j_lower.as_str()) {
                println!("{}", j);
            }
        }
    }
}
