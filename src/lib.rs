extern crate regex;

use regex::Regex;

use std::*;

fn is_operator(token: &str) -> bool {
    "-+*^/@".contains(token)
}

///
/// returns true if token is an operand (simple floating point number)
/// ```
/// assert!(rustingyard::is_operand("4"));
/// ```
///
pub fn is_operand(token: &str) -> bool {
    let re = Regex::new("(\\d*\\.?\\d+)").unwrap();
    re.is_match(token)
}

///
/// returns true if token is a trigonometric or natural log function
///
fn is_function(token: &str) -> bool {
    let re = Regex::new("(sin|cos|tan|ln|arcsin|arccos|arctan)").unwrap();
    re.is_match(token)
}

///
/// defines operator & function precedence
///
fn prec(op: &str) -> i8 {
    match op {
        "+" => 1,
        "-" => 1,
        "*" => 2,
        "/" => 2,
        "^" => 3,
        "@" => 4,
        _ => {
            match is_function(op) {
                true => 5,
                false => {
                    panic!("Invalid operation {}", op);
                },
            }
        },
    }
}

fn is_greater_than_or_equal_precedence(op1: &str, op2: &str) -> bool {
    prec(op1) >= prec(op2)
}

fn space_pad(expression: &str) -> String {
    let rx_set = vec![
        Regex::new("(\\d*\\.?\\d+)").unwrap(),
        Regex::new("([(|)])").unwrap(),
        Regex::new("(sin|cos|tan|ln|arcsin|arccos|arctan)").unwrap(),
        Regex::new("([-|+|*|/|@|^])").unwrap(),
    ];
    let mut res = String::from(expression);
    for rx in rx_set {
        res = rx.replace_all(&res, " $1 ").to_string();
    }
    res = Regex::new("\\s+").unwrap().replace_all(&res, " ").to_string().trim().to_string();
    res
}

fn tokenize<S: Into<String>>(expression: S) -> Vec<String> {
    let unary_minus_rx = Regex::new("((?:^|[-+*/(])\\s*)-").unwrap();
    let converted = unary_minus_rx.replace_all(&expression.into().to_owned(), "$1 @").to_string();

    // TODO this sucks.
    let blah = space_pad(&converted);

    let split = blah.split(" ");

    split
        .into_iter()
        .map(|s| s.to_owned())
        .collect()
}

fn convert2postfix(tokens: Vec<String>) -> Vec<String> {
    let mut stack: Vec<String> = Vec::new();
    let mut result: Vec<String> = Vec::new();
    for token in tokens {
        if is_operand(&token) {
            result.push(token);
        } else if is_operator(&token) || is_function(&token) {
            while !stack.is_empty() && &stack.first().unwrap().as_ref() as &str != "(" && is_greater_than_or_equal_precedence(&stack.first().unwrap(), &token) {
                result.push(stack.pop().unwrap().to_string());
            }
            stack.push(token);
        } else if token == "(" {
            stack.push(token);
        } else if token == ")" {
            while !stack.is_empty() {
                let pop = stack.pop().unwrap();
                if pop.as_ref() as &str != "(" {
                    result.push(pop);
                }
            }
        }
    }

    while !stack.is_empty() {
        result.push(stack.pop().unwrap());
    }
    result
}

///
/// evaluates postfix to a final result
///
fn evaluate_postfix(postfix: Vec<String>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();
    for token in postfix {
        if is_operand(&token) {
            stack.push(token.parse().unwrap());
        } else if is_operator(&token) {
            let op2 = stack.pop().unwrap();
            let op1 = match token.as_ref() {
                "@" => 0.0,
                _ => stack.pop().unwrap(),
            };
            match token.as_ref() {
                "+" => {
                    let result = op1 + op2;
                    stack.push(result);
                }
                "-" => {
                    let result = op1 - op2;
                    stack.push(result);
                }
                "*" => {
                    let result = op1 * op2;
                    stack.push(result);
                }
                "/" => {
                    let result = op1 / op2;
                    stack.push(result);
                }
                "@" => {
                    let result = op2 * -1.0;
                    stack.push(result);
                }
                "^" => {
                    let result = op1.powf(op2);
                    stack.push(result);
                }
                _ => {
                    panic!("Operator {} is not implemented.", token);
                }
            }
        } else if is_function(&token) {
            let op2 = stack.pop().unwrap();
            match token.as_ref() {
                "sin" => {
                    let result = op2.sin();
                    stack.push(result);
                }
                "cos" => {
                    let result = op2.cos();
                    stack.push(result);
                }
                "tan" => {
                    let result = op2.tan();
                    stack.push(result);
                }
                "arcsin" => {
                    let result = op2.asin();
                    stack.push(result);
                }
                "arccos" => {
                    let result = op2.acos();
                    stack.push(result);
                }
                "arctan" => {
                    let result = op2.atan();
                    stack.push(result);
                }
                _ => {
                    panic!("Operator {} is not implemented.", token);
                }
            }
        }
    }
    stack.pop().unwrap()
}

///
/// ```
/// assert_eq!(1, rustingyard::solve("").len());
/// ```
///
pub fn solve<S: Into<String>>(expression: S) -> Vec<f64> {
    let tokens = tokenize(expression.into());
    println!("{}", tokens.len());
    vec![1.0]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, solve("").len());
    }

    #[test]
    fn tokenize_test() {
        assert_eq!(vec!["1", "+", "1", "+", "1"], tokenize("1+1+1"));
        assert_eq!(vec!["1", "-", "1", "-", "1"], tokenize("1-1-1"));
        assert_eq!(vec!["1", "/", "1", "/", "1"], tokenize("1/1/1"));
        assert_eq!(vec!["1", "*", "1", "*", "1"], tokenize("1*1*1"));
        assert_eq!(vec!["1", "+", "1", "*", "1"], tokenize("1+1*1"));
        assert_eq!(vec!["1", "+", "(", "1", "*", "1", ")"], tokenize("1+(1*1)"));
        assert_eq!(vec!["@", "3", "+", "4"], tokenize("-3+4"));
        assert_eq!(vec!["sin", "(", "1", ")"], tokenize("sin(1)"));
        assert_eq!(vec!["2", "*", "sin", "(", "1", ")", "*", "2"], tokenize("2*sin(1)*2"));
        assert_eq!(vec!["3", "^", "2"], tokenize("3^2"));
        assert_eq!(vec!["(", "3", "+", "3", ")", "*", "9"], tokenize("(3+3)*9"));
    }

    #[test]
    fn convert_to_postfix_test() {
        assert_eq!(vec!["1", "1", "+", "1", "+"], convert2postfix(tokenize("1+1+1")));
        assert_eq!(vec!["3", "3", "+", "9", "*"], convert2postfix(tokenize("(3+3)*9")));
        assert_eq!(vec!["4", "2", "-", "5", "*", "10", "+"], convert2postfix(tokenize("((4-2)*5)+10")));
        assert_eq!(vec!["4", "2", "5", "*", "-", "10", "+"], convert2postfix(tokenize("4-2*5+10")));
        assert_eq!(vec!["1", "sin"], convert2postfix(tokenize("sin(1)")));
        assert_eq!(vec!["2", "1", "sin", "*"], convert2postfix(tokenize("2*sin(1)")));
        assert_eq!(vec!["2", "1", "sin", "*", "2", "*"], convert2postfix(tokenize("2*sin(1)*2")));
        assert_eq!(vec!["3", "2", "^"], convert2postfix(tokenize("3^2")));
    }

    #[test]
    fn evaluate_postfix_test() {
        assert_eq!(3.0, evaluate_postfix(convert2postfix(tokenize("1+1+1"))));
        assert_eq!(6.0, evaluate_postfix(convert2postfix(tokenize("2+2+2"))));
        assert_eq!(6.0, evaluate_postfix(convert2postfix(tokenize("1+2+3"))));
        assert_eq!(9.0, evaluate_postfix(convert2postfix(tokenize("3+3+3"))));
        assert_eq!(0.0, evaluate_postfix(convert2postfix(tokenize("0+0+0"))));
        assert_eq!(9.0, evaluate_postfix(convert2postfix(tokenize("3*3"))));
        assert_eq!(4.0, evaluate_postfix(convert2postfix(tokenize("4-2*5+10"))));

        assert_eq!(30.0, evaluate_postfix(convert2postfix(tokenize("3+3*9"))));
        assert_eq!(54.0, evaluate_postfix(convert2postfix(tokenize("(3+3)*9"))));

        assert_eq!(1.0, evaluate_postfix(convert2postfix(tokenize("2-1"))));

        assert_eq!(1.0, evaluate_postfix(convert2postfix(tokenize("-3+4"))));

        assert_eq!(2.0, evaluate_postfix(convert2postfix(tokenize("4/2"))));
        assert_eq!(2.0, evaluate_postfix(convert2postfix(tokenize("4 / 2"))));

        assert_eq!(9.0, evaluate_postfix(convert2postfix(tokenize("3^2"))));

        assert_eq!(0.0_f64, evaluate_postfix(convert2postfix(tokenize(format!("sin({})", std::f64::consts::PI)))).round());
        assert_eq!(0.0_f64, evaluate_postfix(convert2postfix(tokenize("sin(0)"))).round());
        assert_eq!("0.84", format!("{:.2}", evaluate_postfix(convert2postfix(tokenize("sin(1)")))));
        assert_eq!("1.00", format!("{:.2}", evaluate_postfix(convert2postfix(tokenize(format!("sin({})", std::f64::consts::PI / 2.0))))));
        assert_eq!("2.00", format!("{:.2}", evaluate_postfix(convert2postfix(tokenize(format!("2*sin({})", std::f64::consts::PI / 2.0))))));
        assert_eq!("2.00", format!("{:.2}", evaluate_postfix(convert2postfix(tokenize(format!("sin({})*2", std::f64::consts::PI / 2.0))))));
        assert_eq!("4.00", format!("{:.2}", evaluate_postfix(convert2postfix(tokenize(format!("2*sin({})*2", std::f64::consts::PI / 2.0))))));
        assert_eq!("1.57", format!("{:.2}", evaluate_postfix(convert2postfix(tokenize("arcsin(1)")))));
        assert_eq!("0.54", format!("{:.2}", evaluate_postfix(convert2postfix(tokenize("cos(1)")))));
        assert_eq!("-1.00", format!("{:.2}", evaluate_postfix(convert2postfix(tokenize(format!("cos({})", std::f64::consts::PI))))));



    }

    #[test]
    fn is_operator_test() {
        assert!(is_operator("+"));
        assert!(is_operator("-"));
        assert!(is_operator("*"));
        assert!(is_operator("/"));
        assert!(is_operator("^"));
        assert!(is_operator("@"));
    }

    #[test]
    fn space_pad_test() {
        assert_eq!("2 * sin ( 1 ) * 2", space_pad("2*sin(1)*2"));
    }
}
