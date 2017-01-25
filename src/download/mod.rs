pub mod decompressor;

use curl::easy::Easy;

pub fn download_archiv(url: &str) -> Vec<u8> {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url).unwrap();

    // download file and write output in data
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    data
}

// ***** how to use in main.rs *****
//
// let encoded = download_archiv("ftp://ftp.fu-berlin.de/pub/misc/movies/database/complete-crew.\
// list.gz");
// let content = decompress_gz(encoded.as_slice()).unwrap();
//
// let mut string = String::new();
// for c in content {
// string.push(c as char);
// }
//
// println!("{:?}", string);
//
