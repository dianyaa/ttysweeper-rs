mod field;

fn main() {
    let fld: field::Field = field::Field::new(10, 10, 10);
    fld.print_field();
}
