use eframe::egui::{self};

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct CalculatorApp {
    input: String,
    last_op: Option<Operator>,
    result: f64,
    op_pressed_count: u32,
}

// fn clearInput(){

// }
const OPS: [char; 4] = ['+', '-', '*', '/'];
const NUMPAD: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

impl CalculatorApp {
    fn reset_all(&mut self) {
        self.input = String::new();
        self.result = 0.0;
        self.last_op = None;
    }
    fn clear_input(&mut self) {
        self.input = String::new();
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CALCULAR");
            ui.label("This is a Rust GUI app.");
            ui.label("Enter some text:");
            ui.text_edit_singleline(&mut self.input);
            ui.heading(self.result.to_string());
            if ui.button("Clear").clicked() {
                self.reset_all();
            }
            if ui.button("=").clicked() {
                let value: f64 = self.input.trim().parse().expect("parsing input failed");

                if let Some(last_operator) = &self.last_op {
                    match last_operator {
                        Operator::Add => self.result += value,
                        Operator::Divide => self.result /= value,
                        Operator::Multiply => self.result *= value,
                        Operator::Subtract => self.result -= value,
                    };
                }
                self.input = self.result.to_string();
                println!("calculation is {}", self.result);
            }
            // if ui.button("+").clicked(){
            //     self.leftOp = self.input.trim().parse().expect("Please enter a legal number!");
            //     self.result = self.result+self.leftOp;
            //     self.input = String::new();
            // }

            let buttons_size = [50.0, 50.0];

            ui.horizontal(|ui| {
                for op in OPS {
                    if ui
                        .add_sized(buttons_size, egui::Button::new(op.to_string()))
                        .clicked()
                    {
                        let operator = match op {
                            '+' => Operator::Add,
                            '-' => Operator::Subtract,
                            '/' => Operator::Divide,
                            '*' => Operator::Multiply,
                            _ => unreachable!(),
                        };
                        self.last_op = Some(operator);

                        let value: f64 = self.input.trim().parse().expect("Failed parsing input");
                        self.result = value;
                        self.clear_input();
                    }
                }
            });

            ui.horizontal(|ui| {
                if ui.add_sized(buttons_size, egui::Button::new("=")).clicked() {
                    if self.input == "" {
                        return;
                    }

                    let value: f64 = self.input.trim().parse().expect("parsing input failed");

                    if let Some(last_operator) = &self.last_op {
                        match last_operator {
                            Operator::Add => self.result += value,
                            Operator::Divide => self.result /= value,
                            Operator::Multiply => self.result *= value,
                            Operator::Subtract => self.result -= value,
                        };
                    }
                    self.input = self.result.to_string();
                    println!("calculation is {}", self.result);
                    self.last_op = None;
                }
            });
            ui.horizontal(|ui| {
                for num in NUMPAD {
                    if ui
                        .add_sized(buttons_size, egui::Button::new(num.to_string()))
                        .clicked()
                    {
                        self.input.push(num);
                    }
                }
            })
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hello egui",
        options,
        Box::new(|_cc| {
            Box::new(CalculatorApp {
                input: String::new(),
                last_op: None,
                result: 0.0,
                op_pressed_count: 0,
            })
        }),
    )
}
