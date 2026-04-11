use std::io;
use std::io::Write;

fn init() {}

fn run() {
    let mut exit = false;
    let mut saved_certificate: Vec<String> = Vec::new();
    while exit == false {
        exit = home(&mut saved_certificate);
    }
}
fn exit() {}

fn home(saved_certificate: &mut Vec<String>) -> bool {
    clear_screen();
    print!(
        "+--------------------Home---------------------+ \n 
        Create certificate (1)
        Viewe budget (2) 
        Viewe all certificates (3)
        Exit (q)\n"
    );
    return home_input(saved_certificate);
}

fn home_input(saved_certificate: &mut Vec<String>) -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    if input.trim() == "1" {
        println!("+---------------------------------------------+ \n ");
        create_certificate(saved_certificate);
    } else if input.trim() == "2" {
        println!("+---------------------------------------------+ \n ");
    } else if input.trim() == "q" {
        return true;
    } else if input.trim() == "3" {
        clear_screen();
        for s in saved_certificate {
            println!("{}", &s);
        }
    } else {
        println!("wrong\n+---------------------------------------------+ \n ");
    }
    input.clear();
    return false;
}

fn create_certificate(saved_certificate: &mut Vec<String>) {
    clear_screen();
    let mut in_certificate = true;
    let mut col = 0;
    let mut row = 0;
    let mut cert_matrix: Vec<Vec<String>> = Vec::new();
    let cert_start = "+---------------Certificate------------------+ 
| Beskrivning:                               |
+--------------------------------------------+
|Kostnadsställe|     Debit    |    Kredit    |
+--------------------------------------------+
|              |              |              |
+--------------------------------------------+
|                             |              |
+--------------------------------------------+";

    println!("{}", cert_start);
    print!("\x1B[2;15H");
    io::stdout().flush().expect("Failed to flush");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed");
    let mut cert_row: Vec<String> = vec![String::from("0"); 3];
    while in_certificate == true {
        let diff = calc_debit_credit(&cert_matrix);
        let result_x_pos: i32 = 2 * 15 + 2;
        let mut ks = String::new();
        let write_pos = 2 * (row + 4);
        let cell_x = col * 15 + 2;
        let cell_y = (row + 3) * 2;

        print!("\x1B[{};{}H", write_pos, result_x_pos);
        print!("{}", diff.to_string());

        print!("\x1B[{};2H", write_pos);
        println!("                             ");
        print!("\x1B[{};2H", write_pos);
        io::stdout().flush().expect("Failed to flush");
        io::stdin().read_line(&mut ks).expect("Failed");
        let ks_clean = ks.trim().get(1..15).unwrap_or(ks.trim()).to_string();
        print!("\x1B[{};{}H", cell_y, cell_x);
        if ks_clean == ":q" {
            in_certificate = false;
        } else if ks_clean == ":w" {
            store_certificate(&mut cert_matrix, &description, saved_certificate);
            in_certificate = false;
        } else if ks_clean == "\n" {
            print!("0");
            cert_row[col] = "0".to_string();
            if col == 2 {
                cert_matrix.push(cert_row.clone());
            }
            col += 1;
        } else {
            print!("{}", ks_clean);
            cert_row[col] = ks_clean.clone();
            if col == 2 {
                cert_matrix.push(cert_row.clone());
            }
            col += 1;
        }
        if col % 3 == 0 {
            //Changes row when one us filled up
            row += 1;
            col = 0;

            print!("\x1B[{};2H", write_pos);
            print!(
                "              |              |              |
+--------------------------------------------+
|                             |              |
+--------------------------------------------+"
            )
        }
    }
}

fn viewe_budget() {}

fn clear_screen() {
    print!("\x1B[2J");
    print!("\x1B[H")
}
fn store_certificate(
    cert_matrix: &mut Vec<Vec<String>>,
    description: &String,
    saved_certificate: &mut Vec<String>,
) {
    let mut cert_output = format!(
        "+---------------Certificate------------------+\n| Beskrivning: {}",
        description
    );
    cert_output += "+--------------------------------------------+
|Kostnadsställe|     Debit    |    Kredit    |
+--------------------------------------------+\n";
    for row in cert_matrix {
        cert_output += "|";
        for col in row {
            cert_output += &col;
            cert_output += &" ".repeat(14 - col.chars().count());
            cert_output += "|";
        }
        cert_output += &"\n+--------------------------------------------+\n";
    }
    saved_certificate.push(cert_output.clone());
    print!("\x1B[50;1H");
    //print!("{}", &cert_output);
}

fn calc_debit_credit(cert_matrix: &Vec<Vec<String>>) -> i32 {
    let mut debit_zum: i32 = 0;
    let mut credit_zum: i32 = 0;
    for i in 0..cert_matrix.len() {
        debit_zum += cert_matrix[i][1].parse::<i32>().unwrap_or(0);
        credit_zum += cert_matrix[i][2].parse::<i32>().unwrap_or(0);
    }
    return debit_zum - credit_zum;
}

fn main() {
    init();
    run();
    exit();
}
