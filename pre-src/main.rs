extern crate rand;
extern crate termion;
use rand::distributions::{IndependentSample,Range};
use rand::Rng;
use std::io::{Write,stdout};
use std::{char,thread,time};
use termion::{color};
use termion::raw::IntoRawMode;
use termion::cursor::DetectCursorPos;

const MAGIC: &'static str="@";

fn main(){
    let source=format!("{}",MAGIC.replace('\u{40}',MAGIC.replace("\\","\\\\").replace("\"","\\\"").as_str()));
    let mut chars=['\u{3f}';3500];
    assert!(source.len()<=chars.len());
    let mut all_correct=false;
    let mut stdout=stdout().into_raw_mode().unwrap();
    let mut currently_green=false;
    write!(stdout,"{}",color::Fg(color::Red)).unwrap();
    let mut init_draw=true;
    let between=Range::new(32,125+1);
    let mut rng=rand::thread_rng();
    let repaint_period=time::Duration::from_millis(100);

    while !all_correct{
        all_correct=true;
        for (i,(char_s,char_c))in source.chars().zip(chars.iter_mut()).enumerate(){
            if !init_draw&&char_s !=*char_c{
                if rng.next_f32()>0.99{
                    *char_c=char_s;
                }else{
                    *char_c=char::from_u32(between.ind_sample(&mut rng)).unwrap();
                }
            }
            let want_green=char_s==*char_c;
            if !want_green{
                all_correct=false;
            }
            if want_green !=currently_green{
                if want_green{
                    write!(stdout,"{}",color::Fg(color::Green)).unwrap();
                }else{
                    write!(stdout,"{}",color::Fg(color::Red)).unwrap();
                }
                currently_green=want_green;
            }
            write!(stdout,"{}",char_c).unwrap();
            if i%80==79{
                write!(stdout,"\r\n").unwrap();
            }
        }
        if init_draw{
            init_draw=false;
            all_correct=false;
        }
        stdout.flush().unwrap();
        if !all_correct{
            thread::sleep(repaint_period);
            let desired_y=stdout.cursor_pos().unwrap().1-((source.len()-1)/80)as u16;
            write!(stdout,"{}",termion::cursor::Goto(1,desired_y)).unwrap();
        }
    }

    write!(stdout,"\r\n").unwrap();
}
