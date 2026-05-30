// rustfmt-indent_blank_lines: true

fn foo() {
    if true {
        println!("a");

        match 4 {
            0 => {
                println!("b");

                println!("c");
                // FIXME: empty lines between match arms currently ignore this configuration:
            }
        
            x => {
                x = x
                    .wrapping_add({
                        // inner block
                        let a = 4;

                        a + 5
                    })
                    .unwrap();
					  	 	 	 	    	 
                if x > 10 {
                    println!("{x}");
            
                    println!("{x}");
                }
            }
        }
    }
}

fn bar()
where
 	u32: Sized, // FIXME: empty lines between where bounds ignores it
					  	 	 	 	    	 
    i32: Sized,
{}