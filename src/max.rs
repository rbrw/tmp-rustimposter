
extern crate libc;

use libc::c_double;
use libc::size_t;

#[no_mangle]
pub unsafe extern "C" fn doubles_max(c_doubles: *const c_double, n: size_t)
                                     -> c_double
{
    let doubles = std::slice::from_raw_parts(c_doubles, n);
    let mut result = &std::f64::MIN;
    for x in doubles {
        if x > result {
            result = x;
        }
    }
    return *result;
}
