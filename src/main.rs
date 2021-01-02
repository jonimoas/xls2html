use calamine::{open_workbook, Reader, Xlsx};
use warp::{Filter};
use std::env;

#[tokio::main]
async fn main() {
    let port: String = env::var("XLS_PORT").expect("SET XLS_PORT VARIABLE");
    let path = warp::path!(String/String).and_then(render);
    warp::serve(path).run(([127, 0, 0, 1], port.parse().unwrap())).await;
}

async fn render(name: String,sheet:String) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let mut response="<table>".to_string();
    let mut workbook: Xlsx<_> = open_workbook(name).expect("Cannot open file");
    if let Some(Ok(range)) = workbook.worksheet_range(&sheet) {
        for row in range.rows() {
            let mut line="<tr>".to_string();
            for c in row {
                let cell = ["<td>",&c.to_string(),"</td>"].join("");
                line = [line,cell].join("");
            }
            response = [response,line,"</tr>".to_string()].join("");
        }
    }
    Ok(Box::new([response,"</table>".to_string()].join("")))
}
