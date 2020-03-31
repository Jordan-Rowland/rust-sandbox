use std::collections::HashMap;

mod csv_writer;

fn main() {
    // let mut data = Data::open_file("test.csv").expect("Expecting data here");
    // let mut data = Data::open_file("db.csv").expect("Expecting data here");

    // println!("{:?}", data.headers);
    // println!("{:?}", data.rows);
    // println!("{:?}", data.rows_len);
    // println!("{:?}", data.filename);

    // let mut it = true;
    // while it {
    //     let mut name_input = String::new();
    //     println!("Please enter a name");
    //     io::stdin()
    //         .read_line(&mut name_input)
    //         .expect("Name not read");
    //     let mut email_input = String::new();
    //     println!("Please enter an email");
    //     io::stdin()
    //         .read_line(&mut email_input)
    //         .expect("Name not read");
    //     let mut job_title_input = String::new();
    //     println!("Please enter a job title");
    //     io::stdin()
    //         .read_line(&mut job_title_input)
    //         .expect("Name not read");
    //     let mut salary_input = String::new();
    //     println!("Please enter a salary");
    //     io::stdin()
    //         .read_line(&mut salary_input)
    //         .expect("Name not read");

    //     let mut h = HashMap::new();
    //     h.insert("name".to_string(), name_input.trim().to_lowercase());
    //     h.insert("email".to_string(), email_input.trim().to_lowercase());
    //     h.insert(
    //         "job_title".to_string(),
    //         job_title_input.trim().to_lowercase(),
    //     );
    //     h.insert("salary".to_string(), salary_input.trim().to_lowercase());
    //     data.add_row(h);

    //     data.write_csv(None);
    //     it = false;
    // }

    // println!("{:?}", data.headers);

    // println!("{:?}", data.rows);

    // for row in &data.rows {
    //     println!("{:?}", row);
    // }

    // h.insert("name".to_string(), "jimjam".to_string());
    // h.insert("email".to_string(), "jimjam@email.com".to_string());
    // h.insert("job_title".to_string(), "poopman".to_string());
    // h.insert("salary".to_string(), "20000".to_string());

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

    // let k = Vec::new();
    let k = h.keys().to_owned();
    // println!("{:?}", h.keys());
    // println!("{:?}", h.keys());
    d.add_row(h);

    // println!("{:?}", d.rows);
    // println!("{:?}", d.rows_len);
    // println!("{:?}", d.filename.len());

    let mut f_rows = Vec::new();
    f_rows = d
        .get_rows()
        .iter()
        .filter(|row| row.get("name").unwrap() == "jim halpert")
        .collect();
    // println!("{:?}", f_rows);

    let df = csv_writer::Data::from_rows(f_rows);
    println!("{:?}", df.get_rows());
}
