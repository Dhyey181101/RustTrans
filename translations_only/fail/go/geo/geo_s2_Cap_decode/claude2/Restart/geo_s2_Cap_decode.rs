
struct GeoS2Cap {
    center: GeoS2Point,
    radius: f64, 
}

struct GeoS2Point {
    x: f64,
    y: f64,
    z: f64,  
}

struct GeoS2Decoder {
    r: Box<GeoS2ByteReaderImpl>,
    err: Option<std::io::Error>,
    buf: Option<Box<[u8; 8]>>,
}

struct GeoS2ByteReaderImpl;

impl GeoS2ByteReaderImpl {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}

fn decode(cap: &mut GeoS2Cap, d: &mut GeoS2Decoder) {
    cap.center.x = read_f64(d);
    cap.center.y = read_f64(d);
    cap.center.z = read_f64(d);
    cap.radius = read_f64(d); 
}

fn read_f64(d: &mut GeoS2Decoder) -> f64 {
    if d.err.is_some() {
        return 0.0;
    }
    
    let mut b = [0u8; 8];
    let n = buffer(d, &mut b);
    if n != 8 {
        return 0.0;
    }
    
    f64::from_le_bytes(b)
}

fn buffer(d: &mut GeoS2Decoder, buf: &mut [u8]) -> usize {
    if d.buf.is_none() {
        d.buf = Some(Box::new([0u8; 8]));
    }
    
    let src = d.buf.as_mut().unwrap();
    let src_len = src.len();
    
    let dst_len = buf.len();
    let copy_len = dst_len.min(src_len);

    buf[..copy_len].copy_from_slice(&src[..copy_len]);
    copy_len  
}

