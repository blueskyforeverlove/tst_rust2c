#[derive(Debug)]
pub struct RustStruct<'a>
{
    a: Vec<u32>,
    b: &'a str,
    c: Box<String>,
    d: String,
}

#[no_mangle]
pub extern "C" fn RustStruct_create() ->  *mut RustStruct<'static>
{
    let rs = RustStruct {
        a:vec![1,2,3],
        b:"bluesky",
        c:Box::new("zqs".to_string()),
        d:"qingsong".to_string(),
    };
    println!("RustStruct was created");
    Box::into_raw(Box::new(rs))
}

#[no_mangle]
pub unsafe extern "C" fn RustStruct_show(rs: *mut RustStruct)
{
    println!("{:#?}", (*rs));
}

#[no_mangle]
pub unsafe extern "C" fn RustStruct_destroy(rs: *mut RustStruct)
{
    if !rs.is_null() {
        println!("RustStruct was destroyed");
        drop(Box::from_raw(rs));
    }
}