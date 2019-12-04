use plotters::prelude::*;

use piston_window::{EventLoop, PistonWindow, WindowSettings};

use structopt::StructOpt;

use std::process::{Command,Stdio};
use std::collections::vec_deque::VecDeque;

const FPS: u32 = 1;
const LENGTH: u32 = 20;
const N_DATA_POINTS: usize = (FPS * LENGTH) as usize;

#[cfg(target_os = "windows")]
const OS_SPECIFIC_COUNT_ARGUMENT : &str = "-n";

#[cfg(not(target_os = "windows"))]
const OS_SPECIFIC_COUNT_ARGUMENT : &str = "-c";



#[derive(Debug,StructOpt)]
struct Input{
    #[structopt(name="HOST", min_values=1, help="One or more hosts to ping, either as IP or hostname")]
    targets : Vec<String>,
}

fn main() {
    let input = Input::from_args();

    let targets = input.targets;

    println!("Checking delay to {:?}",&targets);

    let mut window: PistonWindow = WindowSettings::new("Delay", [450, 300])
        .samples(4)
        .build()
        .expect("Failed to instantiate");
    window.set_max_fps(FPS as u64);
    let mut epoch = 0;
    let mut data = vec![VecDeque::new(); targets.len()];
    let mut ps = vec!();
    
    println!("Press Ctrl-C to exit");

    // start with 100s and grow if the delay gets higher
    // but dont get smaller
    let mut max_value_ms : u32 = 100;

    while let Some(_) = draw_piston_window(&mut window, |b| {
        let root = b.into_drawing_area();
        root.fill(&WHITE)?;

        if epoch % FPS == 0{
            for target in &targets{
                let ping = Command::new("ping").args(&[OS_SPECIFIC_COUNT_ARGUMENT,"1","-w","1",target])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start ping");

                ps.push(ping)
            }
            
            for (ping, data) in ps.drain(..).zip(data.iter_mut()){
                let out = ping.wait_with_output().expect("Failed to get stdout");


            fn get_delay(reply : &str) -> Option<u32>{
                reply.find("time=")
                    .map(|p1| &reply[p1+"time=".len()..])
                    .map(|d|(d, d.rfind("ms").expect("failed to find ms")))
                    .map(|(d, i)| &d[..i])
                    .map(|d| d.parse().unwrap())
            }

            let stdout = String::from_utf8(out.stdout).expect("Failed to convert to string");
            let delay = stdout.split("\n").filter(|s|s.contains("Reply")).filter_map(get_delay).next().unwrap_or(0);

            if delay  > max_value_ms{
                max_value_ms = delay;
            }

            data.push_back(delay);

            }


        }


        let mut cc = ChartBuilder::on(&root)
            .margin(10)
            .caption("Network Delay", ("sans-serif", 30).into_font())
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_ranged(0..N_DATA_POINTS as u32, 0f32..max_value_ms as f32)?;

        cc.configure_mesh()
            .x_label_formatter(&|x| format!("{}", -(LENGTH as f32) + (*x as f32 / FPS as f32)))
            .y_label_formatter(&|y| format!("{}ms", (*y) as u32))
            .x_labels(15)
            .y_labels(5)
            .x_desc("t")
            .y_desc("Delay [ms]")
            .axis_desc_style(("sans-serif", 15).into_font())
            .draw()?;


        for (idx, data) in (0..).zip(data.iter()) {
            cc.draw_series(LineSeries::new(
                (0..).zip(data.iter()).map(|(a, b)| (a, *b as f32)),
                &Palette99::pick(idx),
            ))?
            .label(format!("Host {}", targets[idx]))
            .legend(move |(x, y)| {
                Rectangle::new([(x - 5, y - 5), (x + 5, y + 5)], &Palette99::pick(idx))
            });
        }

        cc.configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        epoch += 1;
        if epoch as usize > N_DATA_POINTS {
            data.iter_mut().map(|d|d.pop_front()).fold((),|unit,_|unit);
        }

        Ok(())
    }) {}
}
