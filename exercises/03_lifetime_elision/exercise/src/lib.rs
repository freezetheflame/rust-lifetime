use require_lifetimes::require_lifetimes;

#[require_lifetimes(!)]
pub fn example_a<'l>(_number: &'l i32) -> (&'l i32, &'l i32) {
    unimplemented!()
}

#[require_lifetimes(!)]
pub fn example_b<'lifetime1,'lifetime2,'lifetime3>(_first_arg: &'lifetime1 i32, _second_arg: &'lifetime2 i32, _third_arg: &'lifetime3 Option<&'lifetime3 i32>) {
    unimplemented!()
}

#[require_lifetimes(!)]
pub fn example_c<'a>(_first_arg: &'a i32, _second_arg: &'a i32) -> &'a i32 {
    unimplemented!()
}

#[require_lifetimes(!)]
pub fn example_d<'a,'b>(_first_arg: &'a i32, _second_arg: &'b i32) -> &'a i32 {
    unimplemented!()
}
