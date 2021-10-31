pub fn is_executable(mode: u32) -> bool {
    const LIBC_PERMISSION_VALS: [libc::mode_t; 3] = [libc::S_IXUSR, libc::S_IXGRP, libc::S_IXOTH];

    LIBC_PERMISSION_VALS
        .iter()
        .any(|val| mode & (*val as u32) != 0)
}

pub fn mode_to_string(mode: libc::mode_t) -> String {
    const LIBC_FILE_VALS: [(libc::mode_t, char); 7] = [
        (libc::S_IFREG >> 9, '-'),
        (libc::S_IFDIR >> 9, 'd'),
        (libc::S_IFLNK >> 9, 'l'),
        (libc::S_IFSOCK >> 9, 's'),
        (libc::S_IFBLK >> 9, 'b'),
        (libc::S_IFCHR >> 9, 'c'),
        (libc::S_IFIFO >> 9, 'f'),
    ];

    const LIBC_PERMISSION_VALS: [(libc::mode_t, char); 9] = [
        (libc::S_IRUSR, 'r'),
        (libc::S_IWUSR, 'w'),
        (libc::S_IXUSR, 'x'),
        (libc::S_IRGRP, 'r'),
        (libc::S_IWGRP, 'w'),
        (libc::S_IXGRP, 'x'),
        (libc::S_IROTH, 'r'),
        (libc::S_IWOTH, 'w'),
        (libc::S_IXOTH, 'x'),
    ];

    let mut mode_str: String = String::with_capacity(10);
    let mode_shifted = mode >> 9;

    for (val, ch) in LIBC_FILE_VALS.iter() {
        if mode_shifted == *val {
            mode_str.push(*ch);
            break;
        }
    }

    for (val, ch) in LIBC_PERMISSION_VALS.iter() {
        if mode & *val != 0 {
            mode_str.push(*ch);
        } else {
            mode_str.push('-');
        }
    }
    mode_str
}
