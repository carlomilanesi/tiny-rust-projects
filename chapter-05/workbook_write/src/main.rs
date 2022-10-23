use simple_excel_writer::{row, Column, Row, Workbook};

enum Title {
    General,
    Area,
    Account,
}
impl core::fmt::Display for Title {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Title::General => "Sales manager",
                Title::Area => "Area manager",
                Title::Account => "Account manager",
            }
        )
    }
}

struct SalesPerson {
    name: String,
    title: Title,
}
struct SalesAmount {
    person: SalesPerson,
    amount: f64,
}

fn main() {
    let sales = vec![
        SalesAmount {
            person: SalesPerson {
                name: "John Doe".to_string(),
                title: Title::General,
            },
            amount: 730_000.,
        },
        SalesAmount {
            person: SalesPerson {
                name: "Jane Doe".to_string(),
                title: Title::Area,
            },
            amount: 380_000.,
        },
        SalesAmount {
            person: SalesPerson {
                name: "Antony Smith".to_string(),
                title: Title::Account,
            },
            amount: 163_000.,
        },
    ];

    let mut wb = Workbook::create("sales.xlsx");
    let mut sheet = wb.create_sheet("January 2099");
    sheet.add_column(Column { width: 13.0 });
    sheet.add_column(Column { width: 15.0 });
    sheet.add_column(Column { width: 9.0 });
    wb.write_sheet(&mut sheet, |sw| {
        sw.append_row(row!["Name", "Title", "Revenue"])?;
        for sale in &sales {
            sw.append_row(row![
                sale.person.name.to_string(),
                sale.person.title.to_string(),
                sale.amount
            ])?;
        }
        sw.append_blank_rows(1);
        sw.append_row(row!["Total", "", format!("=SUM(C2:C{})", sales.len() + 1)])?;
        Ok(())
    })
    .expect("Error writing worksheet!");
}
