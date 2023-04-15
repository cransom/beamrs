use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;
use image::DynamicImage;
use image::imageops::Lanczos3;
use colored::*;


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
    loops: u32,

    #[arg(short,long, default_value_t = 2)]
    remain: u8,

    #[arg(short,long)]
    file: String,

    #[arg(long, default_value_t = 0)]
    width: u32,
    #[arg(long, default_value_t = 0)]
    height: u32,

    #[arg(long, default_value_t = false)]
    reverse: bool,

    #[arg(long, default_value_t = false)]
    verbose: bool



}
fn main() {

    // cli args
    let args = Args::parse();

    println!("{:?}", args);

    // file setup
    // check for file
    let img = match File::open(&args.file) {
        Ok(f) => f,
        Err(_) => panic!("Couldn't open {}.", args.file)
    };



    let decoder = GifDecoder::new(img).unwrap();
    let mut loop_count: u32 = args.loops;
    let mut frames = decoder.into_frames().collect_frames().expect("Couldn't decode the gif. Is it a gif?");

    // filters?

    if args.reverse {
        frames = frames.into_iter().rev().collect();
    }


    loop {
        for raw_frame in frames.clone() {

            let width;
            let _height;
            let mut frame = DynamicImage::ImageRgba8(raw_frame.buffer() .clone());
            if args.width > 0 && args.height > 0 {
                frame = frame.resize_exact(args.width, args.height, Lanczos3 );
                width = args.width;
                _height = args.height;
            } else {
                width = raw_frame.buffer().width();
                _height = raw_frame.buffer().height();
            }

            let (numer, denom) = raw_frame.delay().numer_denom_ms();
            let mut pixel_sequence: Vec<u8> = vec![];

            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

            for (x, _y, pixel) in frame.to_rgb8().enumerate_pixels() {
                pixel_sequence.extend_from_slice( &[ pixel[0], pixel[1], pixel[2] ]);
                if args.verbose {
                    print!("{}", "#".truecolor(pixel[0], pixel[1], pixel[2]));
                    if x == width-1 {
                        println!("");
                    }
                }
            }
            send_wled(&mut pixel_sequence, &args.target, args.remain);
            thread::sleep(time::Duration::from_millis(numer as u64 / denom as u64 ));
        }
        match loop_count {
            0 => continue,
            1 => break,
            _ => loop_count -= 1
        };


    }


}

fn send_wled(seq: &mut Vec<u8>, host: &String, timeout: u8) {
    // if < 490, drgb = 2. if more, dnrgb
    //drgb
    seq.insert(0,2);
    // timeout
    seq.insert(1,timeout);

    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind a socket");
    socket.set_broadcast(true).expect("couldn't set flag to allow broadcasts");
    socket.send_to(&seq, host).expect("failed to send");


}
