use prost::bytes::{Buf, BufMut, Bytes, BytesMut};

// one byte for the compression flag plus four bytes for the length
const GRPC_HEADER_SIZE: usize = 5;

pub fn encode_body<T>(msg: T) -> fastly::Body
where
    T: prost::Message,
{
    let msg_len = msg.encoded_len();
    let mut buf = BytesMut::with_capacity(GRPC_HEADER_SIZE + msg_len);

    // compression flag, 0 means "no compression"
    buf.put_u8(0);
    buf.put_u32(msg_len as u32);

    msg.encode(&mut buf).unwrap();
    fastly::Body::from(buf.freeze().to_vec())
}

pub fn decode_body<T>(body: fastly::Body) -> T
where
    T: Default + prost::Message,
{
    let mut body: Bytes = body.into_bytes().into();

    // ignore the compression flag
    body.advance(1);

    let len = body.get_u32();
    #[allow(clippy::let_and_return)]
    let msg = T::decode(&mut body.split_to(len as usize)).unwrap();

    msg
}
