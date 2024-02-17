#[derive(Debug)]
pub struct FileStereotype {
    filename: String,
    description: String,
    mime_type: String,
    mime_encoding: String,
    extension: String,
}

impl FileStereotype {
    pub fn new(
        filename: String,
        description: String,
        mime_type: String,
        mime_encoding: String,
        extension: String,
    ) -> FileStereotype {
        FileStereotype {
            filename,
            description,
            mime_type,
            mime_encoding,
            extension,
        }
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn get_mime_type(&self) -> &str {
        &self.mime_type
    }
    pub fn get_mime_encoding(&self) -> &str {
        &self.mime_encoding
    }
    pub fn get_entension(&self) -> &str {
        &self.extension
    }
}
