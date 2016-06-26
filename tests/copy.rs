extern crate fs_utils;
extern crate tempdir;


mod copy_directory {
    use tempdir::TempDir;
    use std::path::{Path, PathBuf};
    use fs_utils::copy_directory;
    fn fixture_at(name: &str) -> PathBuf {
        Path::new(file!()).parent().map(|p| p.join("fixtures").join(name)).unwrap()
    }
    
    #[test]
    fn it_copies_the_content_of_the_entire_directory_recursively_and_respects_basic_permissions() {
        let dest = TempDir::new("dest").unwrap();
        assert_eq!(copy_directory(&fixture_at("source-1"), dest.path()).unwrap(),
        dest.path().join("source-1"))
    }
}

mod destination_dir {
    use fs_utils::destination_dir;
    use std::path::PathBuf;
    
    #[test]
    fn it_always_appends_the_filename_to_destination() {
        assert_eq!(destination_dir("source/subdir", "dest"), PathBuf::from("dest/subdir"));
    }
    
    #[test]
    fn it_can_deal_with_the_root_directory() {
        assert_eq!(destination_dir("/", "dest"), PathBuf::from("dest/ROOT"))
    }
    
    #[test]
    fn it_can_work_with_the_empty_path() {
        assert_eq!(destination_dir("", "dest"), PathBuf::from("dest/fs-utils"));
    }
}