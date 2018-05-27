extern crate rand;
extern crate termion;

use rand::Rng;
use termion::screen::AlternateScreen;
use std::io::{Write, stdout, stdin};
use termion::input::TermRead;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::*;

const COLOUR_RANGE : [color::AnsiValue; 12]  = [color::AnsiValue(196),
    color::AnsiValue(202),color::AnsiValue(208),color::AnsiValue(214),
    color::AnsiValue(220),color::AnsiValue(226),color::AnsiValue(190),
    color::AnsiValue(154),color::AnsiValue(112),color::AnsiValue(70),
    color::AnsiValue(28),color::AnsiValue(22)];

const START_VAL : i64 = 100000;
const DELTA_MOD : i64 = 49;
const DELTA_SLOPE : i64 = 250;
const DIST_MOD : i64 = 15;
const GRID_SIZE : usize = 20;

fn main() {
    let mut field = [[START_VAL as i64; GRID_SIZE]; GRID_SIZE];
    let mut delta = [[0 as i64; GRID_SIZE]; GRID_SIZE];

    let mut currmax = START_VAL;
    let mut currmin = START_VAL;

    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());

    write!(screen, "{}Please give an input. Press 'q' to exit.", cursor::Goto(1,1)).unwrap();

    write!(screen, "{}{}{}", cursor::Goto(3,28), color::Bg(color::Reset), cursor::Hide).unwrap();
    for colour in COLOUR_RANGE.iter(){
        write!(screen, "{}     ", color::Bg(*colour)).unwrap();
    }
    write!(screen, "{}", color::Bg(color::Reset)).unwrap();

    write!(screen, "{}{}|{}{}{}|", cursor::Goto(3,27),clear::CurrentLine,currmin,
        cursor::Goto(3 + 5 * COLOUR_RANGE.len() as u16 - (1 + (currmax as f64).log10().floor() as u16 + 1),27),currmax).unwrap();
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            write!(screen, "{}{}  ", cursor::Goto(5 + 2 * x as u16, 5 + y as u16), 
                   color::Bg(get_colour(currmin,currmax,field[x][y]))).unwrap();
        }
    }

    write!(screen, "{}", color::Bg(color::Reset)).unwrap();
    screen.flush().unwrap();
    for c in stdin.keys() {

        write!(screen, "{}Please give an input. Press 'q' to exit.", cursor::Goto(1,1)).unwrap();
        match c.unwrap() {
            Key::Char('q') => break,
            _ => {}
        }

        screen.flush().unwrap();
        currmax = i64::min_value();
        currmin = i64::max_value();

        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                let deltaval = rand::thread_rng().gen_range(0, 101) - DELTA_MOD;
                delta[x][y] = (field[x][y] / DELTA_SLOPE) * deltaval;
            }
        }

       // write!(screen, "{}{}--{}\n\r{}", screen::ToMainScreen, 1 % 20, -1 % 20,screen::ToAlternateScreen).unwrap();

        write!(screen, "{}{}(0,0) = Value - {}, Delta - {}", cursor::Goto(4,29), clear::CurrentLine, field[0][0], delta[0][0]).unwrap();
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                field[x][y] =  field[x][y] + if delta[x][y] > 0 {(delta[x][y] / 100) * (100 - DIST_MOD * 4)} else {delta[x][y]}
                    + if delta[x][(((y as i64 + 1) % GRID_SIZE as i64) + GRID_SIZE as i64) as usize % GRID_SIZE] > 0
                    {((delta[x][(((y as i64 + 1) % GRID_SIZE as i64) + GRID_SIZE as i64) as usize % GRID_SIZE] / 100) * DIST_MOD)} else {0}
                    + if delta[x][(((y as i64 - 1) % GRID_SIZE as i64) + GRID_SIZE as i64) as usize % GRID_SIZE] > 0
                    {((delta[x][(((y as i64 - 1) % GRID_SIZE as i64) + GRID_SIZE as i64) as usize % GRID_SIZE] / 100) * DIST_MOD)} else {0}
                    + if delta[(((x as i64 + 1) % GRID_SIZE as i64) + GRID_SIZE as i64) as usize % GRID_SIZE][y] > 0
                    {((delta[(((x as i64 + 1) % GRID_SIZE as i64) + GRID_SIZE as i64) as usize % GRID_SIZE][y] / 100) * DIST_MOD)} else {0}
                    + if delta[(((x as i64 - 1) % GRID_SIZE as i64) + GRID_SIZE as i64) as usize % GRID_SIZE][y] > 0
                    {((delta[(((x as i64 - 1) % GRID_SIZE as i64) + GRID_SIZE as i64) as usize % GRID_SIZE][y] / 100) * DIST_MOD)} else {0};
                
                if field[x][y] > currmax {currmax = field[x][y]}
                if field[x][y] < currmin && field[x][y] > DELTA_SLOPE {currmin = field[x][y]}
            }
        }
        
        write!(screen, "{}{}|{}{}{}|", cursor::Goto(3,27),clear::CurrentLine,currmin,
            cursor::Goto(3 + 5 * COLOUR_RANGE.len() as u16 - (1 + (currmax as f64).log10().floor() as u16 + 1),27),currmax).unwrap();

        //write!(screen, "{}{}--{}\n\r{}", screen::ToMainScreen,currmin,currmax,screen::ToAlternateScreen).unwrap();

        for x in 0..20 {
            for y in 0..20 {
        //        write!(screen, "{}{}\n\r{}", screen::ToMainScreen,field[x as usize][y as usize],screen::ToAlternateScreen).unwrap();
                write!(screen, "{}{}  ", cursor::Goto(5 + 2 * x,5 + y), 
                       color::Bg(get_colour(currmin,currmax,field[x as usize][y as usize]))).unwrap();
            }
        }
        write!(screen, "{}", color::Bg(color::Reset)).unwrap();
    }

    write!(screen, "{}{}", cursor::Show, color::Bg(color::Reset)).unwrap();
}

fn get_colour(min :i64, max :i64, value :i64) -> color::AnsiValue {
    let range = max - min;

    if range <= 0 {return color::AnsiValue(226)}
    if value <= DELTA_SLOPE {return color::AnsiValue(0)}

    let size = COLOUR_RANGE.len() as i64;
    let interval = range / size;
    let index = (value - min) / interval;

    if index < size {COLOUR_RANGE[index as usize]}
        else {COLOUR_RANGE[(size - 1) as usize]}
}
