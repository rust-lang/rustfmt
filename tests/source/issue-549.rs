fn main() {
    format!("This is one {} loooooooooooooooooooooooooooooooooooooooooooooooooooooong {} for {}", "very", "string", "test");

    format!("Many arguments: {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", "arg1", "arg2", "arg3", "arg4", "arg5", "arg6", "arg7", "arg8", "arg9", "arg10", "arg11", "arg12", "arg13", "arg14", "arg15");
    
    format!("Many short arguments to test wrap: {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", "arg1", "arg2", "arg3", "arg4", "arg5", "arg6", "arg7", "arg8", "arg9", a, a, a, a, a, a);

    format_args!("This is one {} looooooooooooooooooooooooooooooooooooooooooooooooong {} for {}", "very", "string", "test");
    
    panic!("This is one {} loooooooooooooooooooooooooooooooooooooooooooooooooooooong {} for {}", "very", "string", "test");
    
    print!("This is one {} loooooooooooooooooooooooooooooooooooooooooooooooooooooong {} for {}", "very", "string", "test");
    
    println!("This is one {} loooooooooooooooooooooooooooooooooooooooooooooooooooooong {} for {}", "very", "string", "test");
}
