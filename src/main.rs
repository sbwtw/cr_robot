
mod data_item;

extern crate reqwest;
extern crate serde_json;
extern crate clap;
extern crate time;

use data_item::DataItem;

use std::io::Read;

use serde_json::Value;
use clap::{Arg, App};
use time::strftime;


fn get_data_list(user: &str, page: i32) -> Value {
    let nums_per_page = 50;
    let url = format!(
        "https://cr.deepin.io/changes/?q=owner:{}&S={}&n={}",
        user,
        page * nums_per_page,
        nums_per_page
    );

    println!("Load data: {}", url);

    let mut text = String::new();
    let mut response = reqwest::get(&url).unwrap();
    response.read_to_string(&mut text).unwrap();

    // remove some characters
    serde_json::from_str(&text[4..]).unwrap()
}

fn get_last_monday() -> String {
    let current_time = time::now();
    // time::Tm.tm_wday = 0 ~ 6 for Sun ~ Sat
    let sub_days;
    if current_time.tm_wday == 0 {
        sub_days = 6;
    } else {
        sub_days = current_time.tm_wday - 1;
    }

    let duration = time::Duration::days(sub_days as i64);
    let last_monday = current_time - duration;

    strftime("%F", &last_monday).unwrap()
}

fn dump(list: Vec<DataItem>) {
    let mut last_date = String::new();
    let mut last_project = String::new();
    for item in list {
        if last_date != item.created {
            let tm = time::strptime(&item.created, "%F").unwrap().to_local();

            println!("{}", time::strftime("\n%F %A(%u):", &tm).unwrap());

            last_project.clear();
        }
        last_date = item.created;

        if last_project != item.project {
            println!("\t{}:", item.project);
        }
        last_project = item.project;

        println!("\t\t{}", item.subject);
    }
}

// fn dump_to_json(list: Vec<DataItem>) {
//     let mut last_date: String = "".to_owned();
//     let mut last_project: String = "".to_owned();
//     let mut last_output: String = String::new();
//     let mut output: Vec<String> = vec![];

//     for item in list {
//         if last_date != item.created {
//             last_project.clear();
//             output.push(encode(&last_output).unwrap());
//             last_output.clear();
//         }
//         last_date = item.created;

//         if last_project != item.project {
//             last_output.push_str(&format!("{}:\n", item.project))
//         }
//         last_project = item.project;

//         last_output.push_str(&format!("\t{}\n", item.subject))
//     }
//     // add last data
//     output.push(encode(&last_output).unwrap());
//     // remote first empty data
//     output.remove(0);

//     println!("[{}]", output.join(","));
// }

fn main() {
    let args = App::new("cr_robot")
        .version("1.0")
        .author("sbwtw <sbw@sbw.so>")
        .about("show your code-review record")
        .arg(
            Arg::with_name("username")
                .short("u")
                .long("username")
                .help("your username")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("begin_date")
                .short("b")
                .long("begin_date")
                .help("statistics begin date")
                .takes_value(true),
        )
        // .arg(Arg::with_name("json").short("j").long("json").help(
        //     "output with JSON specification",
        // ))
        .get_matches();

    let user_name = args.value_of("username").unwrap();
    let begin_date: String;
    if let Some(date) = args.value_of("begin_date") {
        begin_date = date.to_owned();
    } else {
        begin_date = get_last_monday();
    }

    //    println!("Statistics from {}.\n", begin_date);

    // save to list
    let mut list: Vec<DataItem> = Vec::new();

    let mut page_num = 0;
    loop {
        let data = get_data_list(user_name, page_num);
        let array = data.as_array().unwrap();
        let mut finished: bool = false;

        'inner: for item in array {
            let di: DataItem = DataItem::new(item);

            if di.created >= begin_date {
                if di.status == "MERGED" {
                    list.push(di);
                }
            } else {
                finished = true;
            }
        }

        if finished || array.is_empty() {
            break;
        } else {
            page_num += 1;
        }
    }

    list.sort();

    // if !to_json {
    dump(list);
    // } else {
    // dump_to_json(list);
    // }
}
