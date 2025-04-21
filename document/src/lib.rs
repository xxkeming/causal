use base64::prelude::*;

// document convert docx, pdf,.. tos markdown
// 从文件中读取数据
pub fn loader_from_data_base64(name: String, data: String) -> Option<(Option<String>, String)> {
    let extensions =
        [".pdf", ".pptx", "docx", ".xls", ".xlsx", ".htm", ".html", ".xml", ".rss", ".atom"];

    if extensions.iter().any(|ext| name.ends_with(ext)) {
        let data = BASE64_STANDARD.decode(data).ok()?;
        let tmp_file = format!("/tmp/{}", name);
        std::fs::write(&tmp_file, data).ok()?;

        let md = markitdown::MarkItDown::new();
        return md.convert(&tmp_file, None).map(|result| (result.title, result.text_content));
    }

    let mime = mime_guess2::from_path(&name).first();
    let Some(mime) = mime else { return None };
    let mime = mime.essence_str();

    // Check if the file name ends with a supported document extension
    if mime.starts_with("text/") {
        return BASE64_STANDARD
            .decode(data)
            .ok()
            .and_then(|data| String::from_utf8(data).ok())
            .map(|text| (None, text));
    }

    return None;
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn pdf_extract() {
    //     let text = pdf_extract::extract_text("/Users/keming/Downloads/jiang.pdf").unwrap();

    //     // 将提取的文本写入到markdown文件
    //     // let path_to_md = "output.md";
    //     // fs::write(path_to_md, text).expect("Unable to write file");

    //     println!("Text has been extracted and saved to {}", text);
    // }

    #[test]
    fn markitdown() {
        let extensions =
            [".pdf", ".doc", "docx", ".xls", ".xlsx", ".htm", ".html", ".xml", ".rss", ".atom"];

        let name = "/Users/keming/Downloads/jiang.pdf";
        if extensions.iter().any(|ext| name.ends_with(ext)) {
            let md = markitdown::MarkItDown::new();
            let result: Option<markitdown::model::DocumentConverterResult> =
                md.convert("/Users/keming/Downloads/jiang.pdf", None);
            println!("Text has been extracted and saved to {:?}", result);
        }

        let mime = mime_guess2::from_path(&name).first();
        let Some(mime) = mime else { return };
        let mime = mime.essence_str();

        println!("mime:{}", mime);

        // let result: Option<markitdown::model::DocumentConverterResult> =
        //     md.convert("/Users/keming/Downloads/jiang.pdf", None);
        // println!("Text has been extracted and saved to {:?}", result);
    }
}
