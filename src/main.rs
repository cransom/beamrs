use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;


use std::{thread, time};
use std::fs::File;
use std::net::UdpSocket;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host
    #[arg(short,long)]
    target: String,

    #[arg(short,long, default_value_t = 0)]
     loops: i32,

    #[arg(short,long, default_value_t = 2)]
    remain: u8,



    #[arg(short,long)]
    file: String,

    #[arg(short,long, default_value = "1x1")]
    dimension: String
}
fn main() {

    // cli args
    let args = Args::parse();

    println!("{:?}, {:?}", args.target, args.file);

    // file setup
    // check for file
    let img = File::open(args.file).unwrap();

    let decoder = GifDecoder::new(img).unwrap();


    let mut loop_count: i32 = args.loops;

    //
    // filters?
    //
    let frames = decoder.into_frames().collect_frames().expect("error decoding");

    loop {
        for frame in frames.clone() {
            let (width, height) = frame.buffer().dimensions();
            let (numer, denom) = frame.delay().numer_denom_ms();
            let mut pixel_sequence: Vec<u8> = vec![];

            for pixel in frame.buffer().pixels() {
                pixel_sequence.extend_from_slice( &[ pixel[0], pixel[1], pixel[2] ]);
            }
            send_wled(&mut pixel_sequence, &args.target);
            thread::sleep(time::Duration::from_millis(numer as u64 / denom as u64 ));
    }
        match loop_count {
            0 => continue,
            1 => break,
            _ => loop_count -= 1
        };


    }


}

fn send_wled(seq: &mut Vec<u8>, host: &String) {

    // if < 490, drgb = 2. if more, dnrgb
    //drgb
    seq.insert(0,2);
    // timeout
    seq.insert(1,2);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind a socket");
    socket.set_broadcast(true).expect("couldn't set flag to allow broadcasts");
    socket.send_to(&seq, host).expect("failed to send");


}
