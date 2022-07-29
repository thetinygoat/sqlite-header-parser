use byteorder::BigEndian;
use byteorder::ByteOrder;
use std::fs;
use std::str;

fn main() {
    let f = fs::read("db").unwrap();
    let header = &f[..100];

    let header_string = str::from_utf8(&header[..16]).unwrap();
    let db_page_size = BigEndian::read_u16(&header[16..18]);
    let write_version = &header[18];
    let read_version = &header[19];
    let reserved_space = &header[20];
    let maximum_payload_fraction = &header[21];
    let minimum_payload_fraction = &header[22];
    let leaf_payload_fraction = &header[23];
    let file_change_counter = BigEndian::read_u32(&header[24..28]);
    let in_header_db_size = BigEndian::read_u32(&header[28..32]);
    let freelist_trunk_page_number = BigEndian::read_u32(&header[32..36]);
    let freelist_page_count = BigEndian::read_u32(&header[36..40]);
    let schema_cookie = BigEndian::read_u32(&header[40..44]);
    let schema_format = BigEndian::read_u32(&header[44..48]);
    let default_page_cache_size = BigEndian::read_u32(&header[48..52]);
    let largest_root_btree_page = BigEndian::read_u32(&header[52..56]);
    let db_text_encoding = BigEndian::read_u32(&header[56..60]);
    let user_version = BigEndian::read_u32(&header[60..64]);
    let incremental_vacuum_mode = BigEndian::read_u32(&header[64..68]);
    let application_id = BigEndian::read_u32(&header[68..72]);
    let version_valid_for = BigEndian::read_u32(&header[92..96]);
    let sqlite_version = BigEndian::read_u32(&header[96..100]);

    println!("header_string = {}", header_string);
    println!("db_page_size = {}", db_page_size);
    println!("write_version = {}", write_version);
    println!("read_version = {}", read_version);
    println!("reserved_space = {}", reserved_space);
    println!("maximum_payload_fraction = {}", maximum_payload_fraction);
    println!("minimum_payload_fraction = {}", minimum_payload_fraction);
    println!("leaf_payload_fraction = {}", leaf_payload_fraction);
    println!("file_change_counter = {}", file_change_counter);
    println!("in_header_db_size = {}", in_header_db_size);
    println!(
        "freelist_trunk_page_number = {}",
        freelist_trunk_page_number
    );
    println!("freelist_page_count = {}", freelist_page_count);
    println!("schema_cookie = {}", schema_cookie);
    println!("schema_format = {}", schema_format);
    println!("default_page_cache_size = {}", default_page_cache_size);
    println!("largest_root_btree_page = {}", largest_root_btree_page);
    println!("db_text_encoding = {}", db_text_encoding);
    println!("user_version = {}", user_version);
    println!("incremental_vacuum_mode = {}", incremental_vacuum_mode);
    println!("application_id = {}", application_id);
    println!("version_valid_for = {}", version_valid_for);
    println!("sqlite_version = {}", sqlite_version);
}
