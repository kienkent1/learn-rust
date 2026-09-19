fn main() {}

//fn get_ref<'a, 'b: 'a>(param_1: &'a i32, param_2: &'b i32) -> &'a i32 {
fn get_ref<'a>(param_1: &'a i32, param_2: &'a i32) -> &'a i32 {
    if param_1 > param_2 {
        param_1
    } else {
        param_2
    }
}
