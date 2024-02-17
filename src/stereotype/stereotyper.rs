use super::file_stereotype::FileStereotype;

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

fn extract_stereotype(filename: &str) -> Result<FileStereotype, Box<dyn std::error::Error>> {
    let cookie = magic::Cookie::open(magic::cookie::Flags::ERROR)?;

    let database = Default::default();
    let cookie = cookie.load(&database)?;

    let summary = cookie.file(filename)?;

    cookie.set_flags(magic::cookie::Flags::ERROR | magic::cookie::Flags::MIME_TYPE)?;
    let mime_type = cookie.file(filename)?;
    cookie.set_flags(magic::cookie::Flags::ERROR | magic::cookie::Flags::MIME_ENCODING)?;
    let mime_encoding = cookie.file(filename)?;

    cookie.set_flags(magic::cookie::Flags::ERROR | magic::cookie::Flags::EXTENSION)?;
    let extension = cookie.file(filename)?;

    let file_stereotype = FileStereotype::new(
        String::from(filename),
        summary,
        mime_type,
        mime_encoding,
        extension,
    );

    Ok(file_stereotype)
}
