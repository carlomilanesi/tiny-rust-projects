use calamine::{open_workbook, Reader, Xlsx};

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        read_workbook(&path);
    }
}

fn read_workbook(path: &str) {
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open workbook file");

    for sheet in workbook.worksheets() {
        println!("=== {} ===", sheet.0);
        for (row_index, row) in sheet.1.rows().skip(1).enumerate() {
            if row.len() < 3 {
                eprint!(
                    "Error: only {} columns at row {}.",
                    row.len(),
                    row_index + 1
                );
                break;
            }
            let name_cell = &row[0];
            if name_cell.is_empty() {
                break;
            }
            let title_cell = &row[1];
            let revenue_cell = &row[2];
            if !name_cell.is_string() {
                eprint!("Error: column 1 at row {} is not a string.", row_index + 1);
                break;
            }
            if !title_cell.is_string() {
                eprint!("Error: column 2 at row {} is not a string.", row_index + 1);
                break;
            }
            if !revenue_cell.is_float() {
                eprint!(
                    "Error: column 3 at row {} is not a floating-point number.",
                    row_index + 1
                );
                break;
            }
            println!(
                "\"{}\", \"{}\", {}",
                name_cell.get_string().unwrap(),
                title_cell.get_string().unwrap(),
                revenue_cell.get_float().unwrap()
            );
        }
    }
}
