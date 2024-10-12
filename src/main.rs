use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder};



fn main() {
    let mut excel: Xlsx<_> = open_workbook("t_sp_parkopedia_parking.xlsx").unwrap();
    if let Ok(r) = excel.worksheet_range("t_sp_parkopedia_parking") {
        for row in r.rows() {
            println!("row={:?}, row[0]={:?}", row, row[0]);
        }
    }
}