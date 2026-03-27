mod macros {
    // 宏定义留在模块内
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    // 把宏导出给整个 crate 使用
    // pub(crate) use my_macro;
}

// 导入宏
// use macros::my_macro;

fn main() {
    my_macro!(); // ✅ 正常运行
}