// compile-flags: -Z parse-only

fn a(B<) {}
   //~^ error: expected one of `:` or `@`, found `<`
