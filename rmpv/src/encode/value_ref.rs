use std::io::Write;

use rmp::encode::write_bool;
use rmp::encode::write_nil;
use rmp::encode::write_sint;
use rmp::encode::{write_uint, write_f32, write_f64, write_str, write_bin, write_array_len, write_map_len, write_ext_meta};

use ValueRef;
use super::Error;

/// Encodes and attempts to write the given non-owning ValueRef into the Write.
///
/// # Errors
///
/// This function returns Error with an underlying I/O error if unable to properly write entire
/// value. Interruption errors are handled internally by silent operation restarting.
///
/// # Examples
/// ```
/// use rmpv::ValueRef;
/// use rmpv::encode::write_value_ref;
///
/// let mut buf = Vec::new();
/// let val = ValueRef::String("le message");
///
/// write_value_ref(&mut buf, &val).unwrap();
/// assert_eq!(vec![0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65], buf);
/// ```
pub fn write_value_ref<W>(wr: &mut W, val: &ValueRef) -> Result<(), Error>
    where W: Write
{
    match *val {
        ValueRef::Nil => {
            write_nil(wr).map_err(|err| Error::InvalidMarkerWrite(err))?;
        }
        ValueRef::Boolean(val) => {
            write_bool(wr, val).map_err(|err| Error::InvalidMarkerWrite(err))?;
        }
        ValueRef::U64(val) => {
            write_uint(wr, val)?;
        }
        ValueRef::I64(val) => {
            write_sint(wr, val)?;
        }
        ValueRef::F32(val) => {
            write_f32(wr, val)?;
        }
        ValueRef::F64(val) => {
            write_f64(wr, val)?;
        }
        ValueRef::String(val) => {
            write_str(wr, val)?;
        }
        ValueRef::Binary(val) => {
            write_bin(wr, val)?;
        }
        ValueRef::Array(ref vec) => {
            write_array_len(wr, vec.len() as u32)?;
            for v in vec {
                write_value_ref(wr, v)?;
            }
        }
        ValueRef::Map(ref map) => {
            write_map_len(wr, map.len() as u32)?;
            for &(ref key, ref val) in map {
                write_value_ref(wr, key)?;
                write_value_ref(wr, val)?;
            }
        }
        ValueRef::Ext(ty, data) => {
            write_ext_meta(wr, data.len() as u32, ty)?;
            wr.write_all(data).map_err(|err| Error::InvalidDataWrite(err))?;
        }
    }

    Ok(())
}
