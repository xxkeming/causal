// document convert docx, pdf,.. tos markdown
pub fn is_loader(name: &str) -> bool {
    let mime = mime_guess2::from_path(&name).first();
    let Some(mime) = mime else { return true };
    let mime = mime.essence_str();

    // Check if the file name ends with a supported document extension
    if mime.starts_with("text/") {
        return true;
    }

    // docx
    if mime == "application/vnd.openxmlformats-officedocument.wordprocessingml.document" {
        return true;
    }

    println!("mime: {:?}", mime);
    false
}

// 从文件中读取数据
pub fn loader_from_file(file: &str) -> Option<String> {
    let mut pandoc = pandoc::new();
    pandoc.add_input(&file);
    pandoc.set_output(pandoc::OutputKind::Pipe);
    pandoc.set_output_format(pandoc::OutputFormat::Markdown, Vec::new());

    if let Ok(pandoc::PandocOutput::ToBuffer(data)) = pandoc.execute() { Some(data) } else { None }
}
