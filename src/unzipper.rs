use std::fs;
use std::path::Path;
use zip::ZipArchive;

// 函数现在接受一个表示压缩文件绝对路径的字符串参数
pub fn unzip_files(zip_file_path: &str) -> zip::result::ZipResult<()> {
    // 直接打开传入的压缩文件
    let file = fs::File::open(zip_file_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let file_name = file.name().to_string();
        let outpath = Path::new(&file_name);
        if file_name.ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            // 确保父目录存在
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            // 创建或覆盖文件
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    println!("压缩文件已解压缩完成。");
    Ok(())
}
