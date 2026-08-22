// 一所虚构的魔法学校里，有一个用 Rust 编写的学生成绩单生成系统！
// 目前，该系统仅支持创建以数字形式（例如，1.0 -> 5.5）来表示学生成绩的成绩单。
// 然而，这所学校也会给出字母形式的成绩（A+ -> F-），
// 我们需要能够打印这两种类型的成绩单！
//
// 请在 `ReportCard` 结构体以及其 impl 实现块中进行必要的代码修改，
// 使系统除了支持数字成绩单之外，也支持字母成绩单。

use std::fmt::Display;

// 在结构体的定义中使用泛型参数`T`
struct ReportCard<T> {
    //           ^^^
    grade: T,
    //     ^
    student_name: String,
    student_age: u8,
}

// 为了能够打印成绩，必须为其实现 `Display` 特征。
impl<T: Display> ReportCard<T> {
    //  ^^^^^^^ 要求泛型 `T` 实现了 `Display`。
    fn print(&self) -> String {
        format!(
            "{} ({}) - 取得的成绩为 {}",
            self.student_name, self.student_age, self.grade,
        )
    }
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "柳如烟".to_string(),
            student_age: 12,
        };
        assert_eq!(report_card.print(), "柳如烟 (12) - 取得的成绩为 2.1",);
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "菠萝吹雪".to_string(),
            student_age: 11,
        };
        assert_eq!(report_card.print(), "菠萝吹雪 (11) - 取得的成绩为 A+",);
    }
}
