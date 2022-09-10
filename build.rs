use std::{env, fs};
use std::collections::BTreeMap;
use std::path::Path;

/// This takes the Unicode files found in resources/ and converts them into the titlecase cable
/// found in casing.rs.
pub fn main() {
    println!("cargo:rerun-if-changed=resources/");
    println!("cargo:rerun-if-changed=src/");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let in_path = Path::new(&env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("resources");
    let sc_path = in_path.join("SpecialCasing.txt");
    let base_path = in_path.join("UnicodeData.txt");
    let dest_path = Path::new(&out_dir).join("casing.rs");

    let mut data: BTreeMap<char, [&str; 3]> = BTreeMap::new();

    let sc_file = fs::read_to_string(sc_path).unwrap();
    sc_file
        .lines()
        .filter(|&s| !s.starts_with('#') && !s.is_empty())
        .for_each(|line| {
            let mut l = line.split(';').take(3).step_by(2);
            let code_point = l.next().unwrap();
            let tcs = l.next().unwrap();
            let tccp: Vec<&str> = tcs.split_ascii_whitespace().collect();
            if let Some(tccp0) = tccp.first().filter(|&&tccp0| code_point != tccp0) {
                let cp = char::from_u32(u32::from_str_radix(code_point, 16).unwrap()).unwrap();
                let tccp1 = tccp.get(1).unwrap_or(&"0");
                let tccp2 = tccp.get(2).unwrap_or(&"0");
                if let Some(old) = data.insert(cp, [tccp0, tccp1, tccp2]) {
                    assert_eq!(&old[0], tccp0);
                    assert_eq!(&old[1], tccp1);
                    assert_eq!(&old[2], tccp2);
                };
            }
        });
    let base_file = fs::read_to_string(base_path).unwrap();
    base_file.lines().for_each(|line| {
        let mut l = line.split(';');
        let cp = l.next().unwrap();
        if let Some(last_cp) = l.last().filter(|&last| !last.is_empty() && cp != last) {
            let cp = char::from_u32(u32::from_str_radix(cp, 16).unwrap()).unwrap();
            if let Some(old) = data.insert(cp, [last_cp, "0", "0"]) {
                assert_eq!(old[0], last_cp, "For code point: {cp}");
                assert_eq!(old[1], "0", "For code point: {cp}");
                assert_eq!(old[2], "0", "For code point: {cp}");
            }
        }
    });

    let lines: String = data
        .iter()
        .map(|(cp, tc)| {
            format!(
                "('\\u{{{:X}}}', ['\\u{{{}}}', '\\u{{{}}}', '\\u{{{}}}',]),\n",
                *cp as u32, tc[0], tc[1], tc[2]
            )
        })
        .collect();

    fs::write(
        dest_path,
        format!("static TITLECASE_TABLE: &[(char, [char; 3])] = &[\n{lines}];"),
    )
    .unwrap();
}
