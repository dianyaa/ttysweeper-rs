mod field;

fn main() {
    let mut fld: field::Field = field::Field::new(10, 10, 10);
    fld.initialize();
    fld.reset();
    fld.print_field();
}
