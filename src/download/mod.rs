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
