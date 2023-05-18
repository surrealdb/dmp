use crate::Dmp;

pub fn fuzz(data: &[u8]) {
    if data.is_empty() {
        return;
    }
    let (flags, rest) = data.split_at(1);

    let flags = flags[0];
    let check_lines = flags & 1 == 1;
    let split = flags >> 1;

    if split as usize > rest.len() {
        return;
    }

    let (a, b) = rest.split_at(split as usize);

    let a = match std::str::from_utf8(a) {
        Ok(s) => s,
        Err(_) => return,
    };

    let b = match std::str::from_utf8(b) {
        Ok(s) => s,
        Err(_) => return,
    };

    // exercise a bunch of functions
    let dmp = Dmp::new();
    dmp.diff_main(a, b, check_lines);
    let _ = dmp.match_main(a, b, flags as i32);
    if let Ok(patches) = dmp.patch_from_text(a.to_string()) {
        dmp.patch_to_text(&patches);

        // don't expect this random patch to succeed but it shouldn't panic
        let _ = dmp.patch_apply(&patches, b);
    }

    // make sure the patches are valid
    let patches = dmp.patch_make1(a, b);
    let (applied, which) = dmp.patch_apply(&patches, a).unwrap();
    let b_patched = applied.into_iter().collect::<String>();
    assert_eq!(b, b_patched);
    assert!(which.into_iter().all(std::convert::identity));
}

#[cfg(test)]
mod tests {
    use super::fuzz;

    #[test]
    fn fuzz_1() {
        let data = &[247, 5, 48, 48, 48, 48, 48, 48, 48, 48, 19, 0, 118, 0, 14, 89, 93, 36, 64, 1, 0, 0, 0, 0, 0, 0, 0, 64, 64, 64, 64, 64, 64, 64, 64, 10, 45, 0, 8, 0, 6, 0, 0, 37, 1, 1, 0, 59, 10, 45, 0, 0, 0, 37, 1, 1, 0, 59, 10, 45, 0, 8, 0, 6, 0, 0, 37, 1, 0, 8, 0, 6, 0, 0, 37, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 59, 10, 45, 0, 0, 0, 37, 1, 1, 0, 59, 10, 45, 0, 8, 0, 6, 0, 0, 37, 1, 0, 52, 52, 0, 0, 0, 10, 0, 52, 0, 0];
        fuzz(data);
    }

    #[test]
    fn fuzz_2() {
        let data = &[247, 5, 48, 48, 48, 48, 48, 14, 91, 93, 36, 64, 1, 0, 0, 0, 0, 0, 0, 0, 64, 64, 64, 64, 64, 24, 64, 64, 64, 10, 45, 0, 8, 0, 6, 0, 0, 37, 4, 1, 0, 59, 10, 45, 0, 49, 0, 0, 37, 1, 1, 0, 59, 10, 45, 0, 8, 0, 6, 2, 0, 37, 1, 0, 8, 0, 6, 0, 0, 37, 1, 1, 0, 59, 10, 45, 0, 0, 59, 37, 1, 0, 0, 1, 10, 45, 0, 8, 0, 6, 0, 0, 37, 1, 0, 8, 0, 6, 0, 0, 37, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 10, 45, 0, 8, 0, 6, 0, 0, 37, 4, 0, 5, 0];
        fuzz(data);
    }
}