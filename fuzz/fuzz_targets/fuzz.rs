#![no_main]
use libfuzzer_sys::fuzz_target;

use dmp::Dmp;

fuzz_target!(|data: &[u8]| {
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

    let dmp = Dmp::new();
    dmp.diff_main(a, b, check_lines);
    let _ = dmp.match_main(a, b, flags as i32);
    if let Ok(patches) = dmp.patch_from_text(a.to_string()) {
        dmp.patch_to_text(&patches);
        //let _ = dmp.patch_apply(&patches, b);
    }
});
