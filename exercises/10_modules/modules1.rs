// TODO: 修复关于调用私有函数的编译器错误。
mod sausage_factory {
    // 不要让此模块之外的任何人看到我们的秘方(`get_secert_recipe`)!
   pub fn get_secret_recipe() -> String {
        let a = String::from("Ginger");
        println!("{}",a);
        a
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
