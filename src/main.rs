use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;


use std::{thread, time};
use std::fs::File;
use std::env;
use std::net::UdpSocket;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host
    #[arg(short,long)]
    host: String


}
fn main() {

    // cli args


    let args: Vec<String> = env ::args().collect();

    let host = &args[1];

    let file = &args[2];

    println!("{}, {}", host, file);

    //
    // udp setup

    //let socket = UdpSocket::send_to
    //
    // file setup
    let img = File::open(file).unwrap();

    let decoder = GifDecoder::new(img).unwrap();

    let mut total_time: u128 = 0;
    let mut loop_count: u32 = 0;
    let mut dir: bool = true;

    //
    // filters?
    //

    // send
    for frame in decoder.into_frames().collect_frames().expect("error decoding gif") {
        let (width, height) = frame.buffer().dimensions();
        let (numer, denom) = frame.delay().numer_denom_ms();
        println!("{}x{}, delay of {}", width, height, numer / denom);
        println!("{}", frame.buffer().pixels().count());
        let mut pixel_sequence: Vec<u8> = vec![];

        for pixel  in frame.buffer().pixels() {
            pixel_sequence.extend_from_slice( &[ pixel[0], pixel[1], pixel[2] ]);
        }
        send_drgb(&mut pixel_sequence, host);
        thread::sleep(time::Duration::from_millis(numer as u64 / denom as u64 ));


    }


}

fn send_drgb(seq: &mut Vec<u8>, host: &String) {
    //drgb
    seq.insert(0,2);
    // timeout
    seq.insert(1,2);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind a socket");
    socket.send_to(&seq, host).expect("failed to send");


}
