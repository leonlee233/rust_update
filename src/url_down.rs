use error_chain::error_chain;
use std::io::copy;
use std::fs::File;
use std::env;


use std::{cmp::min, fmt::Write};

use std::io::Write as;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
pub async fn get_zip(zip_down_path: &str) -> Result<String> {
   let cur_dir = env::current_dir().chain_err(|| "Failed to get current directory")?;
   let target_dir = cur_dir.to_str().ok_or("Failed to convert directory path to string")?;
   let target = zip_down_path; // 传入 ZIP 下载地址

   let response = reqwest::get(target).await?;

   let file_name = {
       let fname = response.url().path_segments()
                           .and_then(|segments| segments.last())
                           .and_then(|name| if name.is_empty() { None } else { Some(name) })
                           .unwrap_or("tmp.bin");
       println!("downloaded: {}", fname);
       format!("{}/{}", target_dir, fname)
   };

/* 
   let mut dest = File::create(&file_name)?;
   let content = response.bytes().await?;
*/

   let total_size = response.content_length().unwrap_or(0);
   let pb = ProgressBar::new(total_size);

   pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

   let mut content = Cursor::new(response.bytes().await?);
   let mut dest = File::create(&file_name)?;
   let mut buffer = [0; 128 * 1024]; // A buffer of 128KB

   while let Ok(n) = content.read(&mut buffer) {
       if n == 0 {
           break;
       }
       dest.write_all(&buffer[..n])?;
       pb.inc(n as u64);
   }

   pb.finish_with_message("Download complete");   



   copy(&mut content.as_ref(), &mut dest)?;
   Ok(file_name)
}


