use super::file_stereotype::FileStereotype;
use gio::{prelude::*, Cancellable};

pub fn stereotype_file(filepath: &str) -> Option<FileStereotype> {
    let result = extract_stereotype(filepath);
    match result {
        Ok(stereotype) => return Some(stereotype),
        Err(err) => {
            println!("{}", err.to_string());
            return None;
        }
    };
}

fn extract_stereotype(filepath: &str) -> Result<FileStereotype, Box<dyn std::error::Error>> {
    let file = gio::File::for_parse_name(filepath);
    let attrs = format!(
        "{},{}",
        gio::FILE_ATTRIBUTE_STANDARD_DISPLAY_NAME,
        gio::FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE
    );
    let file_info = file.query_info(
        attrs.as_str(),
        gio::FileQueryInfoFlags::NONE,
        Cancellable::NONE,
    )?;

    let filename = file_info.display_name();
    let Some(content_type) = file_info.content_type() else {
        return Err("Invalid content type".into());
    };
    let description = gio::functions::content_type_get_description(&content_type);
    let Some(mime_type) = gio::functions::content_type_get_mime_type(&content_type) else {
        return Err("Invalid mime type".into());
    };

    let file_stereotype = FileStereotype::new(
        filename.to_string(),
        filepath.to_string(),
        description.to_string(),
        mime_type.to_string(),
    );

    Ok(file_stereotype)
}
