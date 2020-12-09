fn main() {
    panic!("Foo {:?}", /*"*/
                    1
                );
}
fn main() {
    panic!("Foo {:?}"            /*"*/,
                    1
                );
}
fn main() {
    panic!("Foo {:?}",            /*"*/ 1
                );
}
fn main() {
    panic!("Foo {:?}"             /*"*/, 1
                );
}

fn main() {
	let v = ["A",              /* First Comment */
	"C",             /* Second comment */
	];
}
fn main() {
	let v = ["A"                 /* First Comment */,
	"C",                /* Second comment */
	];
}
fn main() {
	let v = ["A",            /* First Comment */         "C",           /* Second comment */      ];
}
fn main() {
	let v = ["A"          /* First Comment */         , "C",           /* Second comment */           ];
}

fn main() {
	let v = ["A" /* First longgggggggggggggggggggggggggggggggggg Comment */, "C", /* Second comment with some info*/
	];
}
fn main() {
	let v = ["A", /* First longgggggggggggggggggggggggggggggggggg Comment */ "C", /* Second comment with some info*/
	];
}
