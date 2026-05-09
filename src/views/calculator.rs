fn ends_with_digit(s: &str) -> bool {
    s.chars().last().map_or(false, |c| c.is_ascii_digit())
}

pub fn equation_add(equation: &mut String) {
    if ends_with_digit(equation) {
        equation.push('+');
    }
}

pub fn equation_substract(equation: &mut String) {
    if equation.is_empty() || ends_with_digit(equation) {
        equation.push('-');
    }
}

pub fn equation_multiply(equation: &mut String) {
    if ends_with_digit(equation) {
        equation.push('*');
    }
}

pub fn equation_divide(equation: &mut String) {
    if ends_with_digit(equation) {
        equation.push('/');
    }
}

pub fn equation_percent(equation: &mut String) {
    if ends_with_digit(equation) {
        equation.push('%');
    }
}

pub fn equation_dot(equation: &mut String) {
    if ends_with_digit(equation) {
        let last_num_part = equation
            .split(|c: char| !c.is_ascii_digit() && c != '.')
            .last()
            .unwrap_or("");

        if !last_num_part.contains('.') {
            equation.push('.');
        }
    }
}
pub fn calculate_equation(equation: &mut String) -> Option<f64> {
    if equation.is_empty() || !equation.chars().last()?.is_ascii_digit() {
        return None;
    }

    let mut numbers = Vec::new();
    let mut operators = Vec::new();

    fn precedence(op: char) -> i8 {
        match op {
            '+' | '-' => 1,
            '*' | '/' | '%' => 2,
            _ => 0,
        }
    }

    fn apply_op(a: f64, b: f64, op: char) -> Option<f64> {
        match op {
            '+' => Some(a + b),
            '-' => Some(a - b),
            '*' => Some(a * b),
            '/' => {
                if b == 0.0 {
                    None
                } else {
                    Some(a / b)
                }
            }
            '%' => {
                if b == 0.0 {
                    None
                } else {
                    Some(a % b)
                }
            }
            _ => None,
        }
    }

    let tokens: Vec<String> = equation
        .replace('+', " + ")
        .replace('-', " - ")
        .replace('*', " * ")
        .replace('/', " / ")
        .replace('%', " % ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    for token in tokens {
        if let Ok(num) = token.parse::<f64>() {
            numbers.push(num);
        } else {
            let op = token.chars().next()?;
            while !operators.is_empty() && precedence(*operators.last()?) >= precedence(op) {
                let b = numbers.pop()?;
                let a = numbers.pop()?;
                let top_op = operators.pop()?;
                numbers.push(apply_op(a, b, top_op)?);
            }
            operators.push(op);
        }
    }

    while !operators.is_empty() {
        let b = numbers.pop()?;
        let a = numbers.pop()?;
        let op = operators.pop()?;
        numbers.push(apply_op(a, b, op)?);
    }

    numbers.get(0).copied()
}
