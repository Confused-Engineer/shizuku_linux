#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

fn main() 
{
    let lines: Vec<String> = include_str!("../assets/devices")
        .lines()
        .map(String::from)  
        .collect();
    
    loop {
        parse_lines(lines.clone());

        std::thread::sleep(::std::time::Duration::from_secs(5));
    }

}

fn parse_lines(lines: Vec<String>)
{
    for vendor_id in lines{
        #[cfg(debug_assertions)]
        println!("{}", vendor_id);
        
        if let Ok(found) = is_match(vendor_id)
        {
            #[cfg(debug_assertions)]
            println!("{}", found);

            if found
            {
                #[cfg(debug_assertions)]
                println!("Running adb");

                let _ = std::process::Command::new("adb")
                .args(["shell", "sh", "/sdcard/Android/data/moe.shizuku.privileged.api/start.sh"])
                .output();
                
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
        
    }
}


fn is_match(vendor_id: String) -> rusb::Result<bool>
{
    for device in rusb::devices()?.iter()
    {
        let descriptor = device.device_descriptor()?;
        #[cfg(debug_assertions)]
        println!("vendor: {}, descriptor: {:04X}", vendor_id, descriptor.vendor_id());

        if format!("{:04X}", descriptor.vendor_id()) == vendor_id.to_ascii_uppercase()
        {
            return Ok(true);
        }
    }

    Ok(false)
}


//#[cfg(test)]
//mod tests {
//
//
//
//    #[test]
//    fn adb_test() -> Result<(), std::io::Error> {
//        
//        let out = std::process::Command::new("adb")
//            .args(["shell", "sh", "/sdcard/Android/data/moe.shizuku.privileged.api/start.sh"])
//            .stdout(std::process::Stdio::piped())
//            .output()?.stdout;
//
//        let out = String::from_utf8(out);
//        
//        match out {
//            Ok(out) => println!("{}", out),
//            Err(_) => println!("err"),
//        }
//        Ok(())
//    }
//
//
//
//    #[test]
//    fn assets()
//    {
//        let test: Vec<String> = include_str!("../assets/devices")
//                .lines()
//                .map(String::from)  
//                .collect();
//
//        for line in test
//        {
//            println!("{}", line)
//        }
//    }
//
//
//
//}//