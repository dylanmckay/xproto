use crate::bindings::*;

use protocol::{self, Parcel};

#[test]
fn client_prefix_is_right_size() {
    let client_prefix = xConnClientPrefix {
        byteOrder: 1,
        pad: 1,
        majorVersion: 11,
        minorVersion: 0,
        nbytesAuthProto: 0,
        nbytesAuthString: 0,
        pad2: 0,
    };

    let raw_bytes = client_prefix.raw_bytes(&protocol::Settings::default()).unwrap();
    assert_eq!(12, raw_bytes.len());
}
