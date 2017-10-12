extern crate vorbis;
extern crate hound;
extern crate byteorder;

use std::io::Write;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

fn main() {
    let mut args = std::env::args();
    args.next();
    // It needs 3 file address as arguments:
    //      first for input wav,
    //      second for vorbis output file,
    let error = "Error: Usage is <in-pcm-file> <out-vorbis-file>";
    let in_file = args.next().expect(error);
    let out_file = args.next().expect(error);
    let mut out_file = std::fs::File::create(out_file).unwrap();

    let mut reader = hound::WavReader::open(&in_file).unwrap();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    let spec = reader.spec();

    println!("channels: {:?}", spec.channels);
    println!("rate: {:?}", spec.sample_rate);
    println!("----");

    let mut encoder = vorbis::Encoder::new(spec.channels as u8, spec.sample_rate as u64, vorbis::VorbisQuality::Midium).expect("Error in creating encoder");

    let samples_enc = encoder.encode(&samples).expect("Error in encoding.");

    analyze_samples(samples_enc.as_slice());

    out_file.write(samples_enc.as_slice()).expect("Error in writing");
    out_file.write(encoder.flush().expect("Error in flushing.").as_slice()).expect("Error in writing");
}

fn analyze_samples(samples: &[u8]) {
    let mut counter = 0;
    let mut rdr = Cursor::new(samples);

    while counter < 6 {
        counter += 1;

        let capture_pattern = rdr.read_u32::<BigEndian>().unwrap();
        println!("Pattern: {:X}", capture_pattern);
        let version = rdr.read_u8().unwrap();
        println!("Version: {:X}", version);
        let header_type = rdr.read_u8().unwrap();
        println!("Header Type: {:X}", header_type);
        let position = rdr.read_u64::<LittleEndian>().unwrap();
        println!("Position: {:X}", position);
        let bitstream_seq = rdr.read_u32::<LittleEndian>().unwrap();
        println!("Bitstream Serial: {:X}", bitstream_seq);
        let page_seq = rdr.read_u32::<LittleEndian>().unwrap();
        println!("Page Serial: {:X}", page_seq);
        let checksum = rdr.read_u32::<BigEndian>().unwrap();
        println!("Checksum: {:X}", checksum);
        let num_segments = rdr.read_u8().unwrap();
        println!("Segment count: {}", num_segments);

        let mut segment_table = Vec::with_capacity(num_segments as usize);
        for i in 0..num_segments {
            let segment_size = rdr.read_u8().unwrap();
            segment_table.push(segment_size);
            println!("Segment {} size: {}", i, segment_size);
        }

        let sum = segment_table.iter().fold(0i64, |a,b| a + (*b as i64));
        rdr.seek(SeekFrom::Current(sum));

        println!("----");

        // read segments
/*        for i in 0..num_segments {
            let mut data = Vec::new();
            rdr.take(segment_table[i as usize] as u64).read(&mut data);
            println!("Data: {:?}", data);
        }*/
    }
}