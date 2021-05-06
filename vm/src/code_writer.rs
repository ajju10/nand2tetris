use std::fs::File;
use std::io::{BufWriter, Write};

pub struct CodeWriter {
    label_count: usize,
    writer: BufWriter<File>,
    file_name: String,
}

impl CodeWriter {
    pub fn new(output_file: File, file_name: &str) -> Self {
        Self {
            label_count: 0,
            writer: BufWriter::new(output_file),
            file_name: String::from(file_name),
        }
    }

    /// Convenience method for writing Assembly code
    fn write(&mut self, code: &str) {
        self.writer.write(format!("{}\n", code).as_bytes()).unwrap();
    }

    /// Bootstrap Code
    pub fn write_init(&mut self) {
        self.write("// Bootstrap Code");
        self.write("@256");
        self.write("D=A");
        self.write("@SP");
        self.write("M=D");
        self.write_call("Sys.init", 0);
        self.write("// Bootstrap code ends");
        self.write("");
    }

    pub fn write_function(&mut self, function_name: &str, num_locals: usize) {
        self.write(&format!("// function {} {}", function_name, num_locals));
        self.write(&format!("({})", function_name));

        for _ in 0..num_locals {
            self.write_push_pop("push", "constant", 0);
        }
    }

    pub fn write_call(&mut self, function_name: &str, no_of_args: usize) {
        self.label_count += 1;
        let new_label = &format!("RETURN_LABEL{}", self.label_count);
        self.write(&format!("// call function {}", function_name));
        self.write(&format!("@{}", new_label));
        self.write("D=A");
        self.write_push_d_onto_stack();
        self.write_push_pop("push", "local", 0);
        self.write_push_pop("push", "argument", 0);
        self.write_push_pop("push", "this", 0);
        self.write_push_pop("push", "that", 0);

        self.write("@SP");
        self.write("D=M");
        self.write("@5");
        self.write("D=D-A");
        self.write(&format!("@{}", no_of_args));
        self.write("D=D-A");
        self.write("@ARG");
        self.write("M=D");
        self.write("@SP");
        self.write("D=M");
        self.write("@LCL");
        self.write("M=D");
        self.write(&format!("@{}", function_name));
        self.write("0;JMP");
        self.write(&format!("({})", new_label));
    }

    /// Assembly code for return command
    pub fn write_return(&mut self) {
        self.write("// return");
        self.write("@LCL");
        self.write("D=M");
        self.write("@R11");
        self.write("M=D");
        self.write("@5");
        self.write("A=D-A");
        self.write("D=M");
        self.write("@R12");
        self.write("M=D");
        self.write_push_pop("pop", "argument", 0);
        self.write("@ARG");
        self.write("D=M");
        self.write("@SP");
        self.write("M=D+1");
        self.write_pre_frame_template("THIS");
        self.write_pre_frame_template("THAT");
        self.write_pre_frame_template("ARG");
        self.write_pre_frame_template("LCL");
        self.write("@R12");
        self.write("A=M");
        self.write("0;JMP");
    }

    fn write_pre_frame_template(&mut self, segment: &str) {
        self.write(&format!("// pre-frame {}", segment));
        self.write("@R11");
        self.write("D=M-1");
        self.write("AM=D");
        self.write("D=M");
        self.write(&format!("@{}", segment));
        self.write("M=D");
    }

    pub fn write_label(&mut self, label: &str) {
        self.write(&format!("// label {}", label));
        self.write(&format!("({})", label))
    }

    pub fn write_goto(&mut self, label: &str) {
        self.write(&format!("// goto {}", label));
        self.write(&format!("@{}", label));
        self.write("0;JMP")
    }

    pub fn write_if_goto(&mut self, label: &str) {
        self.write(&format!("// if-goto {}", label));
        self.write("@SP");
        self.write("AM=M-1");
        self.write("D=M");
        self.write("A=A-1");
        self.write(&format!("@{}", label));
        self.write("D;JNE")
    }

    pub fn write_arithmetic(&mut self, token: &str) {
        self.write(&format!("// {}", token));
        if token == "add" {
            self.write_pop_stack_into_d();
            self.write_decrement_stack_pointer();
            self.write_load_stack_pointer_to_a();
            self.write("M=D+M");
            self.write_increment_stack_pointer();
        } else if token == "sub" {
            self.write_pop_stack_into_d();
            self.write_decrement_stack_pointer();
            self.write_load_stack_pointer_to_a();
            self.write("M=M-D");
            self.write_increment_stack_pointer();
        } else if token == "and" {
            self.write_pop_stack_into_d();
            self.write_decrement_stack_pointer();
            self.write_load_stack_pointer_to_a();
            self.write("M=D&M");
            self.write_increment_stack_pointer();
        } else if token == "or" {
            self.write_pop_stack_into_d();
            self.write_decrement_stack_pointer();
            self.write_load_stack_pointer_to_a();
            self.write("M=D|M");
            self.write_increment_stack_pointer();
        } else if token == "not" {
            self.write_decrement_stack_pointer();
            self.write_load_stack_pointer_to_a();
            self.write("M=!M");
            self.write_increment_stack_pointer();
        } else if token == "neg" {
            self.write_decrement_stack_pointer();
            self.write_load_stack_pointer_to_a();
            self.write("M=-M");
            self.write_increment_stack_pointer();
        } else if token == "eq" {
            self.write_compare_operation("JEQ");
        } else if token == "gt" {
            self.write_compare_operation("JGT");
        } else if token == "lt" {
            self.write_compare_operation("JLT");
        } else {
            panic!("Can't recognize arithmetic command")
        }
    }

    pub fn write_push_pop(&mut self, command: &str, segment: &str, value: i32) {
        if command.contains("push") {
            self.write(&format!("// push {} {}", segment, value));
            if segment == "constant" {
                // Store value in D
                self.write(&format!("@{}", value));
                self.write("D=A");
            } else if segment == "local" {
                self.write_load_segment("LCL", value);
                self.write("D=M");
            } else if segment == "this" {
                self.write_load_segment("THIS", value);
                self.write("D=M");
            } else if segment == "that" {
                self.write_load_segment("THAT", value);
                self.write("D=M");
            } else if segment == "argument" {
                self.write_load_segment("ARG", value);
                self.write("D=M");
            } else if segment == "pointer" {
                self.write(&format!("@R{}", 3 + value));
                self.write("D=M");
            } else if segment == "temp" {
                self.write(&format!("@R{}", 5 + value));
                self.write("D=M");
            } else if segment == "static" {
                let symbol = self.file_name.clone();
                let symbol: Vec<&str> = symbol
                    .split(".")
                    .collect();
                self.write(&format!("@{}.{}", symbol[0], value));
                self.write("D=M");
            } else {
                panic!("Cannot recognize command")
            }
            self.write_push_d_onto_stack();
        } else {
            self.write(&format!("// pop {} {}", segment, value));
            if segment == "constant" {
                self.write(&format!("@{}", value));
            } else if segment == "local" {
                self.write_load_segment("LCL", value);
            } else if segment == "argument" {
                self.write_load_segment("ARG", value);
            } else if segment == "this" {
                self.write_load_segment("THIS", value);
            } else if segment == "that" {
                self.write_load_segment("THAT", value);
            } else if segment == "pointer" {
                self.write(&format!("@R{}", 3 + value));
            } else if segment == "temp" {
                self.write(&format!("@R{}", 5 + value));
            } else if segment == "static" {
                let symbol = self.file_name.clone();
                let symbol: Vec<&str> = symbol
                    .split(".")
                    .collect();
                self.write(&format!("@{}.{}", symbol[0], value));
            } else {
                panic!("Cannot recognize command")
            }
            self.write("D=A");
            self.write("@R13");
            self.write("M=D");
            self.write_pop_stack_into_d();
            self.write("@R13");
            self.write("A=M");
            self.write("M=D");
        }
    }

    fn write_compare_operation(&mut self, jump_command: &str) {
        self.write_pop_stack_into_d();
        self.write_decrement_stack_pointer();
        self.write_load_stack_pointer_to_a();
        self.write("D=M-D");
        self.write(&format!("@LABEL{}", self.label_count));
        self.write(&format!("D;{}", jump_command));
        self.write_load_stack_pointer_to_a();

        self.write("M=0");
        self.write(&format!("@ENDLABEL{}", self.label_count));
        self.write("0;JMP");
        self.write(&format!("(LABEL{})", self.label_count));
        self.write_load_stack_pointer_to_a();

        self.write("M=-1");
        self.write(&format!("(ENDLABEL{})", self.label_count));
        self.write_increment_stack_pointer();
        self.label_count += 1
    }

    fn write_pop_stack_into_d(&mut self) {
        self.write_decrement_stack_pointer();
        self.write("A=M");
        self.write("D=M");
    }

    fn write_push_d_onto_stack(&mut self) {
        self.write_load_stack_pointer_to_a();
        self.write("M=D");
        self.write_increment_stack_pointer();
    }

    fn write_increment_stack_pointer(&mut self) {
        self.write("@SP");
        self.write("M=M+1");
    }

    fn write_decrement_stack_pointer(&mut self) {
        self.write("@SP");
        self.write("M=M-1");
    }

    fn write_load_stack_pointer_to_a(&mut self) {
        self.write("@SP");
        self.write("A=M");
    }

    fn write_load_segment(&mut self, segment: &str, value: i32) {
        self.write(&format!("@{}", segment));
        self.write("D=M");
        self.write(&format!("@{}", value));
        self.write("A=D+A");
    }
}
