use std::collections::HashMap;

mod csv_writer;
// use csv_writer::Filename;

fn main() {
    let mut d = csv_writer::Data::new(
        String::from("test.csv"),
        vec![
            String::from("id"),
            String::from("name"),
            String::from("email"),
        ],
    );

    let mut h = HashMap::<String, String>::new();
    h.insert(d.get_headers()[0].clone(), "128".to_string());
    h.insert(d.get_headers()[1].clone(), "jim halpert".to_string());
    h.insert(
        d.get_headers()[2].clone(),
        "jim@dundermifflin.com".to_string(),
    );
    d.add_row(h);

    let mut h = HashMap::<String, String>::new();
    h.insert(d.get_headers()[0].clone(), "233".to_string());
    h.insert(d.get_headers()[1].clone(), "Dwight Schrute".to_string());
    h.insert(d.get_headers()[2].clone(), "DS@sabre.com".to_string());
    d.add_row(h);

    let mut h = HashMap::<String, String>::new();
    h.insert(d.get_headers()[0].clone(), "019".to_string());
    h.insert(d.get_headers()[1].clone(), "michael scott".to_string());
    h.insert(
        d.get_headers()[2].clone(),
        "Michael@dundermifflin.com".to_string(),
    );

    d.add_row(h);
    let dc = d.get_column("email");

    let df = csv_writer::Data::from_rows(d.find_rows_by_column("name".into(), "ch".into()));

    let dr = df.get_rows();
    println!("{:?}", dr);

    let fc = d.find_rows_by_column("name".into(), "ch".into());
    println!("{:?}", fc);
}
