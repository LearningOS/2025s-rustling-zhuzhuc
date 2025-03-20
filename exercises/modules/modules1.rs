// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.



mod sausage_factory {
    // 私有函数，外部模块无法访问
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // bug修复：添加 pub 关键字，让外部模块可以访问该函数
    pub fn make_sausage() {
        let recipe = get_secret_recipe();
        println!("使用秘方: {} 制作香肠", recipe);
        println!("sausage!");
    }
}

fn main() {
    // 调用香肠工厂模块的制作香肠函数
    sausage_factory::make_sausage();
}
