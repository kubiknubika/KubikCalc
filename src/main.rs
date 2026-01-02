mod errors;
mod engine;

use std::io::{self, Write};
use engine::KubikEngine;

fn print_help() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║         KubikCalc v4.0 (Advanced Engineering)                ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    
    println!(" ❓ КАК ЭТИМ ПОЛЬЗОВАТЬСЯ (RPN):");
    println!("    Здесь нет скобок. Сначала пишутся числа, потом действие.");
    println!("    ❌ Обычный стиль:  (5 + 3) * 2");
    println!("    ✅ Стиль Kubik:    5 3 + 2 *");
    
    println!("\n 🛠  ПОЛНЫЙ СПИСОК КОМАНД:");
    println!(" ───────────────────────────────────────────────────────────────");
    println!(" [Базовые]      +   -   *   /   ^   recip (1/x)");
    println!(" [Тригоном.]    sin  cos  tan  asin acos atan");
    println!(" [Гипербол.]    sinh cosh tanh");
    println!(" [Углы]         deg (в градусы)   rad (в радианы)");
    println!(" [Логарифмы]    ln (base e)       log (base 10)  log_base (y x)");
    println!(" [Алгебра]      sqrt (корень 2)   root (корень y из x)");
    println!("                ! (факториал)     abs (модуль)");
    println!(" [Округление]   round  ceil  floor");
    println!(" [Константы]    pi   e");
    println!(" [Управление]   clear  drop  exit");
    println!("────────────────────────────────────────────────────────────────");
}

fn main() {
    print_help();

    let mut calculator = KubikEngine::new();
    let stdin = io::stdin();

    loop {
        let current_stack = calculator.get_stack();
        
        // Визуально отделяем ввод от вывода пустой строкой
        if current_stack.is_empty() {
            print!("\nStack [ ] > ");
        } else {
            print!("\nStack {:?} > ", current_stack);
        }
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Ошибка чтения");
        let input = input.trim();

        // Выход
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("q") {
            println!("👋 Bye!");
            break;
        }
        // Повтор справки
        if input.eq_ignore_ascii_case("help") {
            print_help();
            continue;
        }
        if input.is_empty() { continue; }

        for token in input.split_whitespace() {
            if let Err(e) = calculator.eval(token) {
                println!("  ❌ Ошибка: {:?}", e);
            }
        }
        
        if let Some(last) = calculator.get_stack().last() {
            println!("   = {:.4}", last);
        }
    }
}