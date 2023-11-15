mod url_down;
mod unzipper;
use url_down::get_zip;

#[tokio::main]
async fn main() {
    let mut files_name= String::new();
    match get_zip("url").await {
        Ok(file_name) => {
            println!("Downloaded to {}", file_name);
            files_name = file_name;
        }
        Err(e) => eprintln!("Error: {}", e),
    }

   
    match unzipper::unzip_files(&files_name) {
        Ok(_) => println!("解压成功"),
        Err(e) => println!("解压失败: {:?}", e),
    }
}