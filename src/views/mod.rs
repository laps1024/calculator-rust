use egui::{
    Button, CentralPanel, Color32, FontId, Layout, Margin, Panel, RichText, TextStyle,
    ViewportCommand,
};
mod calculator;
#[derive(Default)]
pub struct App {
    equation: String,
}
#[derive(PartialEq, Eq)]
enum Action {
    Add,
    Substract,
    Divide,
    Multiply,
    Digit(u8),
    Clear,
    Percent,
    Equals,
    Dot,
    Remove,
    Link,
}

impl Action {
    fn execute(&mut self, equation: &mut String) {
        if *equation == "Ошибка вычисления!" {
            *equation = String::new();
        }
        match self {
            Action::Add => calculator::equation_add(equation),
            Action::Substract => calculator::equation_substract(equation),
            Action::Divide => calculator::equation_divide(equation),
            Action::Multiply => calculator::equation_multiply(equation),
            Action::Digit(num) => equation.push_str(format!("{}", num).as_str()),
            Action::Clear => *equation = String::new(),
            Action::Percent => calculator::equation_percent(equation),
            Action::Equals => {
                if let Some(val) = calculator::calculate_equation(equation) {
                    *equation = format!("{}", val)
                } else {
                    *equation = format!("Ошибка вычисления!")
                }
            }
            Action::Dot => calculator::equation_dot(equation),
            Action::Remove => {
                equation.pop();
            }
            Action::Link => {
                webbrowser::open("https://github.com/laps1024").expect("Не удалось открыть ссылку!")
            }
        }
    }
}
struct ActionButton {
    button: egui::Button<'static>,
    action: Action,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        Self::render_top_panel(ui);
        self.render_central_panel(ui);
    }
}
impl App {
    fn render_top_panel(ui: &mut egui::Ui) {
        Panel::top("menu").show_inside(ui, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Max), |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        ui.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
                ui.add_space(ui.available_width() - 20.0);
                if ui.menu_button("X", |_ui| {}).response.clicked() {
                    ui.send_viewport_cmd(ViewportCommand::Close);
                }
            });
        });
    }
    fn render_central_panel(&mut self, ui: &mut egui::Ui) {
        let height = ui.available_height() / 6.0;
        let width = ui.available_width() / 4.0 - 5.0;
        let left_right_layout = Layout::left_to_right(egui::Align::Min);
        let right_left_layout = Layout::right_to_left(egui::Align::Min);
        self.draw_panel(ui);
        self.draw_number_grid(ui, [width, height], left_right_layout, right_left_layout);
    }
    fn draw_number_grid(
        &mut self,
        ui: &mut egui::Ui,
        size: [f32; 2],
        left_right_layout: Layout,
        right_left_layout: Layout,
    ) {
        CentralPanel::default().show_inside(ui, |ui| {
            ui.style_mut().spacing.item_spacing.x = 3.0;
            ui.style_mut().spacing.item_spacing.y = 3.0;
            let tool_row = vec![
                Self::create_button("C".to_string(), Action::Clear),
                Self::create_button("<".to_string(), Action::Remove),
                Self::create_button("%".to_string(), Action::Percent),
                Self::create_button("/".to_string(), Action::Divide),
            ];
            let mut row_one = Self::create_number_buttons(7..=9u8);
            row_one.push(Self::create_button("*".to_string(), Action::Multiply));
            let mut row_two = Self::create_number_buttons(4..=6u8);
            row_two.push(Self::create_button("-".to_string(), Action::Substract));
            let mut row_three = Self::create_number_buttons(1..=3u8);
            row_three.push(Self::create_button("+".to_string(), Action::Add));
            let row_four = vec![
                Self::create_button("=".to_string(), Action::Equals),
                Self::create_button(".".to_string(), Action::Dot),
                Self::create_button("0".to_string(), Action::Digit(0)),
                Self::create_button("Creator".to_string(), Action::Link),
            ];
            let rows = vec![tool_row, row_one, row_two, row_three];
            for row in rows {
                self.render_row(ui, row, size, left_right_layout);
            }
            self.render_row(ui, row_four, size, right_left_layout);
        });
    }
    fn draw_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading(
            RichText::new("Calculator")
                .color(Color32::WHITE)
                .background_color(Color32::TRANSPARENT),
        );
        egui::Frame::new()
            .inner_margin(Margin {
                left: 15,
                right: 15,
                ..Default::default()
            })
            .fill(Color32::WHITE)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_height(30.0);
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.heading(
                        RichText::new(&self.equation)
                            .color(Color32::BLACK)
                            .background_color(Color32::TRANSPARENT),
                    )
                });
            });
    }
    fn render_row(
        &mut self,
        ui: &mut egui::Ui,
        buttons: Vec<ActionButton>,
        size: [f32; 2],
        layout: Layout,
    ) {
        ui.with_layout(layout, |ui| {
            for mut action_button in buttons {
                if ui.add_sized(size, action_button.button).clicked() {
                    action_button.action.execute(&mut self.equation);
                }
            }
        });
    }
    fn create_button(caption: String, action: Action) -> ActionButton {
        let button_color = match action {
            Action::Link => Color32::from_rgb(160, 28, 28),
            Action::Digit(0) => Color32::from_rgb(62, 62, 62),
            Action::Clear | Action::Remove | Action::Divide | Action::Percent => {
                Color32::from_rgb(28, 100, 28)
            }
            Action::Multiply | Action::Substract | Action::Add | Action::Equals | Action::Dot => {
                Color32::from_rgb(28, 28, 100)
            }
            _ => Color32::from_rgb(82, 82, 82),
        };
        ActionButton {
            button: Button::new(caption).fill(button_color),
            action,
        }
    }
    fn create_number_buttons(range: std::ops::RangeInclusive<u8>) -> Vec<ActionButton> {
        let mut buttons: Vec<ActionButton> = Vec::new();
        if *range.start() > *range.end() {
            let range = *range.end()..=*range.start();
            for i in range.rev() {
                buttons.push(Self::create_number_button(i));
            }
        } else {
            for i in range {
                buttons.push(Self::create_number_button(i));
            }
        }
        buttons
    }
    fn create_number_button(number: u8) -> ActionButton {
        ActionButton {
            button: Button::new(format!("{}", number)),
            action: Action::Digit(number),
        }
    }
    pub fn set_styles(ctx: &egui::Context) {
        let mut styles = (*ctx.global_style()).clone();
        styles.text_styles = [
            (
                TextStyle::Body,
                FontId::new(25.0, egui::FontFamily::Monospace),
            ),
            (
                TextStyle::Button,
                FontId::new(45.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Heading,
                FontId::new(28.0, egui::FontFamily::Proportional),
            ),
            (
                TextStyle::Small,
                FontId::new(20.0, egui::FontFamily::Monospace),
            ),
        ]
        .into();
        styles.visuals.widgets.inactive.fg_stroke.color = Color32::WHITE;
        ctx.set_global_style(styles);
    }
}
