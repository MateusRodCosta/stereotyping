#[derive(Debug)]
pub struct FileStereotype {
    filename: String,
    filepath: String,
    description: String,
    mime_type: String,
}

impl FileStereotype {
    pub fn new(
        filename: String,
        filepath: String,
        description: String,
        mime_type: String,
    ) -> FileStereotype {
        FileStereotype {
            filename,
            filepath,
            description,
            mime_type,
        }
    }

    pub fn get_filename(&self) -> &str {
        &self.filename
    }
    pub fn get_filepath(&self) -> &str {
        &self.filepath
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn get_mime_type(&self) -> &str {
        &self.mime_type
    }
}
