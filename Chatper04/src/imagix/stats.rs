use super::error::ImagixError;
use super::resize::get_image_files;
use std::path::Path;

pub fn get_stats<P: AsRef<Path>>(src_folder: P) -> Result<(usize, f64), ImagixError> {
    let image_files = get_image_files(src_folder)?;
    let size = image_files
        .iter()
        .map(move |f| f.metadata().unwrap().len())
        .sum::<u64>();
    Ok((image_files.len(), (size / 1000000) as f64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_get_stats() {
        let path = PathBuf::from("/tmp/images");
        let (count, size) = get_stats(path).unwrap();
        // Note: For this test to pass,
        // alter the count and size with the right values
        assert_eq!(count, 2);
        assert_eq!(size, 17.0);
    }
}
