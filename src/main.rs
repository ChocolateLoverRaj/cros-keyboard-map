use std::{fmt::format, fs, io, path::Iter, str::SplitWhitespace};

fn vivaldi_scan_code_to_keyd(scan_code: &str) -> Result<&str, ()> {
    match scan_code {
        "90" => Ok("previoussong"),
        "91" => Ok("zoom"),
        "92" => Ok("scale"),
        "93" => Ok("print"),
        "94" => Ok("brightnessdown"),
        "95" => Ok("brightnessup"),
        "97" => Ok("kbdillumdown"),
        "98" => Ok("kbdillumup"),
        "99" => Ok("nextsong"),
        "9A" => Ok("playpause"),
        "9B" => Ok("micmute"),
        "9E" => Ok("kbdillumtoggle"),
        "A0" => Ok("mute"),
        "AE" => Ok("volumedown"),
        "B0" => Ok("volumeup"),
        "E9" => Ok("forward"),
        "EA" => Ok("back"),
        "E7" => Ok("refresh"),
        _ => Err(()),
    }
}

fn load_physmap_scan_codes() -> Result<Vec<String>, io::Error> {
    match fs::read_to_string("/sys/bus/platform/devices/i8042/serio0/function_row_physmap") {
        Ok(contents) => Ok(contents.split_whitespace().map(|s| s.to_owned()).collect()),
        Err(e) => Err(e),
    }
}

fn create_keyd_config(physmap: Vec<&str>) -> String {
    let p1 = "\
[ids]
0001:0001

[main]
";
    // make fn keys act like vivaldi keys when super isn't held
    let vivaldi_for_non_vivaldi: Vec<_> = physmap
        .iter()
        .enumerate()
        .map(|(i, scan_code)| {
            let keyd = vivaldi_scan_code_to_keyd(scan_code).unwrap();
            let f_number = i + 1;
            format!("f{f_number} = ") + if keyd == "zoom" { "f11" } else { keyd }
        })
        .collect();
    let vivaldi = &(vivaldi_for_non_vivaldi.join("\n") + "\n");

    // make vivaldi keys act like vivaldi keys when super isn't held
    let vivaldi_for_vivaldi: Vec<_> = physmap
        .iter()
        .map(|scan_code| {
            let vivaldi = vivaldi_scan_code_to_keyd(scan_code).unwrap();
            let mapping = if vivaldi == "zoom" { "f11" } else { vivaldi };
            format!("{vivaldi} = {mapping}")
        })
        .collect();
    let vivaldi_for_vivaldi = &(vivaldi_for_vivaldi.join("\n") + "\n");

    // map lock button to coffee
    let lock = "\
f13 = coffee
sleep = coffee
";

    let meta = "\n[meta]\n";
    // make fn keys act like fn keys when super is held
    let meta_mappings_non_vivaldi: Vec<_> = physmap
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let f_number = i + 1;
            format!("f{f_number} = f{f_number}")
        })
        .collect();
    let meta_mappings_non_vivaldi = &(meta_mappings_non_vivaldi.join("\n") + "\n");
    // make vivaldi keys act like like fn keys when super is held
    let meta_mappings_vivaldi: Vec<_> = physmap
        .iter()
        .enumerate()
        .map(|(i, scan_code)| {
            let vivaldi = vivaldi_scan_code_to_keyd(&scan_code).unwrap();
            let f_number = i + 1;
            format!("{vivaldi} = f{f_number}")
        })
        .collect();
    let meta_mappings_vivaldi = &(meta_mappings_vivaldi.join("\n") + "\n");
    let extra_shortcuts = "
[alt]
backspace = delete
brightnessdown = kbdillumdown
brightnessup = kbdillumup
f6 = kbdillumdown
f7 = kbdillumup

[control]
f5 = print
scale = print

[control+alt]
backspace = C-A-delete
";

    "".to_owned()
        + p1
        + vivaldi
        + vivaldi_for_vivaldi
        + lock
        + meta
        + meta_mappings_non_vivaldi
        + meta_mappings_vivaldi
        + extra_shortcuts
}

fn main() {
    let config = create_keyd_config(
        load_physmap_scan_codes()
            .unwrap()
            .iter()
            .map(|scan_code| scan_code.as_str())
            .collect(),
    );
    print!("{}", config);
    fs::write("cros.conf", config).unwrap();
}
