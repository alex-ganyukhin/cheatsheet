use crate::commands::command::{
    CommandErrorProcessor, CommandOutputAndErrorProcessor, CommandOutputProcessor, CommandOutputVariant,
};


pub struct PlaintextCommandOutputAndErrorProcessor {
    cout: Box<dyn std::io::Write>,
    cerr: Box<dyn std::io::Write>,
}


impl PlaintextCommandOutputAndErrorProcessor {
    pub fn new(cout: Box<dyn std::io::Write>, cerr: Box<dyn std::io::Write>) -> Self {
        Self { cout: cout, cerr: cerr }
    }
}

impl Default for PlaintextCommandOutputAndErrorProcessor {
    fn default() -> Self {
        Self {
            cout: Box::new(std::io::stdout()),
            cerr: Box::new(std::io::stderr()),
        }
    }
}


const INDENT_STEP: &str = "  ";


/// Prints the given output variant to the specified destination in a plaintext format with given indentation level.
/// - If variant is just a value, it is printed as is and nothing else.
/// - If variant is an array, each element is printed on a new line with increased indentation
/// - If variant is a dictionary, each key-value pair is printed on a new line with increased indentation for values
fn print(dst: &mut dyn std::io::Write, output: &CommandOutputVariant, level: usize) {
    match output {
        CommandOutputVariant::Value(val) => {
            let indent = INDENT_STEP.repeat(level);
            let _ = write!(dst, "{}{}", indent, val);
        }
        CommandOutputVariant::Array(arr) => {
            for item in arr {
                print(dst, item, level + 1);
            }
        }
        CommandOutputVariant::Dict(dict) => {
            for (key, value) in dict {
                let indent = INDENT_STEP.repeat(level);

                let _ = writeln!(dst, "{}{}", indent, key);
                print(dst, value, level + 1);
            }
        }
    }

    let _ = write!(dst, "\n");
}


impl CommandOutputProcessor for PlaintextCommandOutputAndErrorProcessor {
    fn process_output(&mut self, output: &CommandOutputVariant) {
        print(&mut *self.cout, output, 0usize);
    }
}


impl CommandErrorProcessor for PlaintextCommandOutputAndErrorProcessor {
    fn process_error(&mut self, error: &anyhow::Error) {
        spdlog::error!("Error: {}", error);

        let mut source = error.source();
        while let Some(inner) = source {
            spdlog::error!("Caused by: {}", inner);
            source = inner.source();
        }

        let _ = writeln!(self.cerr, "Error: {}", error);
    }
}


impl CommandOutputAndErrorProcessor for PlaintextCommandOutputAndErrorProcessor {}
