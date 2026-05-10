use ratatui::layout::Size;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct SavedCert {
    description: String,
    date: Vec<u32>,
    cert: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct CertStore {
    certificates: Vec<SavedCert>,
}
#[derive(Serialize, Deserialize, Debug)]
struct AcountsList {
    acount_list: Vec<Acount>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Acount {
    name: String,
    balance: i32,
}

fn init() {}

fn run() {
    let mut exit = false;
    while exit == false {
        exit = home();
    }
}
fn exit() {}

fn home() -> bool {
    clear_screen();
    print!(
        "+--------------------Home---------------------+ \n 
        Create certificate (1)
        Add acount (2)
        Viewe acounts (3)
        Viewe budget (4) 
        Viewe all certificates (5)
        Exit (q)\n"
    );
    return home_input();
}

fn home_input() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    if input.trim() == "1" {
        println!("+---------------------------------------------+ \n ");
        create_certificate();
    } else if input.trim() == "2" {
        println!("+---------------------------------------------+ \n ");
        add_acount();
    } else if input.trim() == "q" {
        return true;
    } else if input.trim() == "5" {
        clear_screen();
        vew_all_certificate();
    } else if input.trim() == "3" {
        clear_screen();
        viewe_accounts();
    } else {
        println!("wrong\n+---------------------------------------------+ \n ");
    }
    input.clear();
    return false;
}

fn create_certificate() {
    //======Setup================
    clear_screen();
    let mut date = Vec::<String>::new();
    let mut description = String::new();
    let cert_start = "+---------------Certificate------------------+ 
| Beskrivning:                               |
+--------------------------------------------+
|    Datum:    20  /  /                      |
+--------------------------------------------+
|Kostnadsställe|     Debit    |    Kredit    |
+--------------------------------------------+
|              |              |              |
+--------------------------------------------+
|                             |              |
+--------------------------------------------+";

    println!("{}", cert_start);

    //========Downloading jason==============
    let mut stored_acounts: AcountsList = if let Ok(content) = fs::read_to_string("acounts.json") {
        serde_json::from_str(&content).unwrap_or(AcountsList {
            acount_list: vec![],
        })
    } else {
        AcountsList {
            acount_list: vec![],
        }
    };

    let mut avalaebul_acounts = Vec::new();

    for i in &stored_acounts.acount_list {
        avalaebul_acounts.push(i.name.to_lowercase());
    }

    //============="Beskrivning"===================
    print!("\x1B[2;15H");
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut description).expect("Failed");

    if description.trim() == ":q" {
        home();
    }

    //=============="År"====================
    print!("\x1B[4;18H");
    io::stdout().flush().expect("Failed to flush");
    date.push(String::new());
    io::stdin().read_line(&mut date[0]).expect("Failed");

    //=============="Månad"====================
    print!("\x1B[4;21H");
    io::stdout().flush().expect("Failed to flush");
    date.push(String::new());
    io::stdin().read_line(&mut date[1]).expect("Failed");

    //=============="Dag"====================
    print!("\x1B[4;24H");
    io::stdout().flush().expect("Failed to flush");
    date.push(String::new());
    io::stdin().read_line(&mut date[2]).expect("Failed");

    //==============Certmatrix===============
    cert_matrix(&avalaebul_acounts);
    //     let in_certificate = true;
    //     let mut col = 0;
    //     let mut row = 0;
    //     let mut cert_matrix: Vec<Vec<String>> = Vec::new();
    //
    //     let mut cert_row: Vec<String> = vec![String::from("0"); 3];
    //     let mut acount_name = String::new();
    //     while in_certificate == true {
    //         let diff = calc_debit_credit(&cert_matrix);
    //         let result_x_pos: i32 = 2 * 15 + 2;
    //         let mut ks = String::new();
    //         let write_pos = 2 * (row + 5);
    //         let cell_x = col * 15 + 2;
    //         let cell_y = (row + 4) * 2;
    //
    //         print!("\x1B[{};{}H", write_pos, result_x_pos);
    //         print!("{}", diff.to_string());
    //
    //         print!("\x1B[{};2H", write_pos);
    //         println!("                             ");
    //         print!("\x1B[{};2H", write_pos);
    //         io::stdout().flush().expect("Failed to flush");
    //         io::stdin().read_line(&mut ks).expect("Failed");
    //         let input_clean = ks.trim().get(1..15).unwrap_or(ks.trim()).to_string();
    //         print!("\x1B[{};{}H", cell_y, cell_x);
    //         if input_clean == ":q" {
    //             home();
    //         } else if input_clean == ":w" {
    //             let finnished_cert = make_certificate(&mut cert_matrix, &description, &date);
    //             let date_int: Vec<u32> = vec![
    //                 date[0].trim().parse().expect("Fel"),
    //                 date[1].trim().parse().expect("Fel"),
    //                 date[2].trim().parse().expect("Fel"),
    //             ];
    //             let stored_cert = SavedCert {
    //                 description: description.clone(),
    //                 date: date_int.clone(),
    //                 cert: finnished_cert,
    //             };
    //
    //             let mut store: CertStore = if let Ok(content) = fs::read_to_string("certificates.json")
    //             {
    //                 serde_json::from_str(&content).unwrap_or(CertStore {
    //                     certificates: vec![],
    //                 })
    //             } else {
    //                 CertStore {
    //                     certificates: vec![],
    //                 }
    //             };
    //
    //             store.certificates.push(stored_cert);
    //
    //             let json_data = serde_json::to_string_pretty(&store).expect("Kunde ej skapa JSON");
    //
    //             fs::write("certificates.json", json_data).expect("Kunde inte spara filen");
    //             break;
    //         } else {
    //             if col == 0 {
    //                 if !avalaebul_acounts.contains(&input_clean.to_lowercase()) {
    //                 } else {
    //                     print!("{}", input_clean);
    //                     acount_name = input_clean.clone();
    //                 }
    //                 col += 1;
    //             }
    //         }
    //         if col == 0 {
    //         } else if input_clean == "\n" {
    //             print!("0");
    //             cert_row[col] = "0".to_string();
    //             if col == 2 {
    //                 cert_matrix.push(cert_row.clone());
    //             }
    //             col += 1;
    //         } else {
    //             print!("{}", input_clean);
    //             for acount in &mut stored_acounts.acount_list {
    //                 if acount.name == acount_name.trim() {
    //                     acount.balance += input_clean.trim().parse::<i32>().expect("NaN");
    //                 }
    //             }
    //             cert_row[col] = input_clean.clone();
    //             if col == 2 {
    //                 cert_matrix.push(cert_row.clone());
    //             }
    //             col += 1;
    //         }
    //         if col % 3 == 0 && col != 0 {
    //             //Changes row when one us filled up
    //             row += 1;
    //             col = 0;
    //
    //             print!("\x1B[{};2H", write_pos);
    //             print!(
    //                 "              |              |              |
    // +--------------------------------------------+
    // |                             |              |
    // +--------------------------------------------+"
    //             )
    //         }
    //         let json_data = serde_json::to_string_pretty(&stored_acounts).expect("Kunde ej skapa JSON");
    //
    //         fs::write("acounts.json", json_data).expect("Kunde inte spara filen");
    //     }
}

fn cert_matrix(available_accounts: &Vec<String>) {
    let col = 0;
    let row = 0;
    let mut cert_matrix: Vec<Vec<String>> = Vec::new();

    let mut cert_row: Vec<String> = vec![String::from("0"); 3];

    let mut pos_matrix: Vec<Vec<(u32, u32)>> = Vec::new();

    pos_matrix.push(vec![
        (cell_x(0), cell_y(0)),
        (cell_x(1), cell_y(0)),
        (cell_x(2), cell_y(0)),
    ]);
    recurtion_start_for_cel_matrix(
        &mut pos_matrix,
        col,
        row,
        &mut cert_matrix,
        &mut cert_row,
        available_accounts,
    );
}

fn recurtion_start_for_cel_matrix(
    pos_matrix: &mut Vec<Vec<(u32, u32)>>,
    curent_col: u32,
    curent_row: u32,
    cert_matrix: &mut Vec<Vec<String>>,
    cert_row: &mut Vec<String>,
    available_accounts: &Vec<String>,
) {
    let mut col = curent_col;
    let mut row = curent_row;
    if curent_col == 3 {
        cert_matrix.push(cert_row.clone());
        print!("\x1B[{};2H", write_pos(row));
        print!(
            "              |              |              |
+--------------------------------------------+
|                             |              |
+--------------------------------------------+"
        );
        col = 0;
        row = curent_row + 1;
    }
    let result_x_pos: i32 = 2 * 15 + 2;
    let write_pos = 2 * (row + 5);
    let diff = calc_debit_credit(&cert_matrix);
    print!("\x1B[{};{}H", write_pos, result_x_pos);
    print!("{}", diff.to_string());

    from_input_to_cel(
        pos_matrix,
        col,
        row,
        cert_matrix,
        cert_row,
        &available_accounts,
    );
}

fn from_input_to_cel(
    pos_matrix: &mut Vec<Vec<(u32, u32)>>,
    curent_col: u32,
    curent_row: u32,
    cert_matrix: &mut Vec<Vec<String>>,
    cert_row: &mut Vec<String>,
    available_accounts: &Vec<String>,
) {
    let mut input = String::new();

    go_to_input(write_pos(curent_row));
    io::stdin().read_line(&mut input).expect("Failed");

    let input_clean = input.trim().get(1..15).unwrap_or(input.trim()).to_string();
    if input_clean == ":q" {
        home();
    } else if input_clean == ":w" {
        //write_to_mem();
    } else if input_clean == "^[[A" {
        //clear_last_cert_row();
    } else if input_clean == "^[[D" {
        //date_day();
    } else {
        if curent_col == 0 {
            if !available_accounts.contains(&input_clean.to_lowercase()) {
                from_input_to_cel(
                    pos_matrix,
                    curent_col,
                    curent_row,
                    cert_matrix,
                    cert_row,
                    available_accounts,
                );
            } else {
                write_to_this_cel(curent_row, curent_col, &input_clean);
                cert_row[curent_col as usize] = input_clean.clone();
                recurtion_start_for_cel_matrix(
                    pos_matrix,
                    curent_col + 1,
                    curent_row,
                    cert_matrix,
                    cert_row,
                    available_accounts,
                );
            }
        } else {
            if input_clean == "\n" {
                write_to_this_cel(curent_row, curent_col, &"0".to_string());
                cert_row[curent_col as usize] = "0".to_string();
                recurtion_start_for_cel_matrix(
                    pos_matrix,
                    curent_col + 1,
                    curent_row,
                    cert_matrix,
                    cert_row,
                    available_accounts,
                );
            } else if !input_clean.parse::<u32>().is_ok() {
                from_input_to_cel(
                    pos_matrix,
                    curent_col,
                    curent_row,
                    cert_matrix,
                    cert_row,
                    available_accounts,
                );
            } else {
                write_to_this_cel(curent_row, curent_col, &input_clean);
                cert_row[curent_col as usize] = input_clean;
                recurtion_start_for_cel_matrix(
                    pos_matrix,
                    curent_col + 1,
                    curent_row,
                    cert_matrix,
                    cert_row,
                    available_accounts,
                );
            }
        }
    }
}

fn cell_x(col: u32) -> u32 {
    return col * 15 + 2;
}
fn cell_y(row: u32) -> u32 {
    return (row + 4) * 2;
}
fn write_pos(row: u32) -> u32 {
    return 2 * (row + 5);
}

fn go_to_input(write_pos: u32) {
    print!("\x1B[{};2H", write_pos);
    println!("                             ");
    print!("\x1B[{};2H", write_pos);
    io::stdout().flush().expect("Failed to flush");
}
fn write_to_this_cel(row: u32, col: u32, output: &String) {
    print!("\x1B[{};{}H", cell_y(row), cell_x(col));
    print!("{}", output);
}

fn add_acount() {
    clear_screen();
    let mut acount_name = String::from("");
    let mut balance_input = String::from("");
    let balance: i32;
    let acount_display = String::from(
        "+---------------Nytt konto------------------+ 
| Kontonamn:                                 |
+--------------------------------------------+
|   Saldo:                                   |
+--------------------------------------------+",
    );

    println!("{}", acount_display);

    print!("\x1B[2;14H");
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut acount_name).expect("Failed");
    if acount_name.len() > 15 {
        add_acount();
    } else if acount_name == ":q" {
        home();
    }

    print!("\x1B[4;14H");
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut balance_input).expect("Failed");
    if balance_input == ":q" {
        home();
    }

    balance = balance_input.trim().parse::<i32>().expect("NaN");

    let new_acount = Acount {
        name: acount_name.trim().to_owned(),
        balance: balance,
    };

    let mut store: AcountsList = if let Ok(content) = fs::read_to_string("acounts.json") {
        serde_json::from_str(&content).unwrap_or(AcountsList {
            acount_list: vec![],
        })
    } else {
        AcountsList {
            acount_list: vec![],
        }
    };

    store.acount_list.push(new_acount);

    let json_data = serde_json::to_string_pretty(&store).expect("Kunde ej skapa JSON");

    fs::write("acounts.json", json_data).expect("Kunde inte spara filen");
    home();
}

fn viewe_accounts() {
    let stored_accounts: AcountsList = if let Ok(content) = fs::read_to_string("acounts.json") {
        serde_json::from_str(&content).unwrap_or(AcountsList {
            acount_list: vec![],
        })
    } else {
        AcountsList {
            acount_list: vec![],
        }
    };
    println!("+----------------Accounts------------------+");
    for i in stored_accounts.acount_list {
        println!(
            "| {}                                {} |",
            i.name.trim(),
            i.balance
        );
    }

    io::stdout().flush().expect("Kunde inte flusha stdout");

    let mut pause_buffer = String::new();
    io::stdin()
        .read_line(&mut pause_buffer)
        .expect("Fel vid paus");
}

fn viewe_budget() {}

fn vew_all_certificate() {
    let store: CertStore = if let Ok(content) = fs::read_to_string("certificates.json") {
        serde_json::from_str(&content).unwrap_or(CertStore {
            certificates: vec![],
        })
    } else {
        CertStore {
            certificates: vec![],
        }
    };
    for i in store.certificates {
        println!("{}", i.cert);
    }
    io::stdout().flush().expect("Kunde inte flusha stdout");

    let mut pause_buffer = String::new();
    io::stdin()
        .read_line(&mut pause_buffer)
        .expect("Fel vid paus");
}

fn clear_screen() {
    print!("\x1B[2J");
    print!("\x1B[H")
}
fn make_certificate(
    cert_matrix: &mut Vec<Vec<String>>,
    description: &String,
    date: &Vec<String>,
) -> String {
    let date_str = format!("20{}/{}/{}", date[0].trim(), date[1].trim(), date[2].trim());

    let mut cert_output = format!(
        "+---------------Certificate------------------+\n| Beskrivning: {}+--------------------------------------------+\n|    Datum:    {}\n",
        description, date_str
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
    return cert_output;
    //print!("\x1B[50;1H");
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
