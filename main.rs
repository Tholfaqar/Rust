use std::collections::HashMap;

mod loans;

pub fn main(){
    let mut assets: Vec<i32> =Vec::new(); 
    let mut debts: Vec<i32> = Vec::new();

    //let mut assets_hm:HashMap<String, i32> = HashMap::new();
    //let mut debts_hm:HashMap<String, i32> = HashMap::new();

    let mut assets_hm: HashMap<&String, i32> = HashMap::new();
    let mut debts_hm: HashMap<&String, i32> = HashMap::new();

    assets.push(20);
    assets.push(30);

    debts.push(5);
    debts.push(10);

    loans::calc_cashflow(assets, debts);
}

pub fn add_debt(debts_hm: &mut HashMap<&String, i32>, debt_info: String, debt_amount: i32){
    debts_hm.insert(debt_info, debt_amount);
}

pub fn add_income(assets_hm: &mut HashMap<&String, i32>, income_info: String, income_amount: i32){
    assets_hm.insert(income_info, income_amount);
}

pub fn init_assets(mut assets_hm: HashMap<String, i32>){
    let mut info = String::from("rental 1");
    let mut amount: i32;
    amount = 2500;
    
    add_income(assets_hm, info, amount);
    /*
    asset_hm.insert(String::from(), 2500);
    asset_hm.insert(String::from("rental 2"), 2700);
    asset_hm.insert(String::from("devidand c1"), 500);
    asset_hm.insert(String::from("devidand c2"), 300);
    asset_hm.insert(String::from("salary 1"), 6000);
    */

}

pub fn init_debt(mut debts_hm: HashMap<String, i32>){
    /*
    let mut info = String::from("property tax 1");
    let mut amount: i32;
    amount = 400;
    add_debt(debts_hm, info, amount);

    info = String::from("property tax 2");
    amount = 500;
    add_debt(&debts_hm, info, amount);
    
    info = String::from("mortgage 1");
    amount = 1200;
    add_debt(debts_hm, info, amount);

    info = String::from("mortgage 2");
    amount = 1300;
    add_debt(debts_hm, info, amount);

    info = String::from("home mortgage");
    amount = 1500;
    add_debt(debts_hm, info, amount);
    */
    
    /*
    debts_hm.insert(String::from("property tax 1"), 400);
    debts_hm.insert(String::from("property tax 2"), 500);
    debts_hm.insert(String::from("mortgage 1"), 1200);
    debts_hm.insert(String::from("mortgage 2"), 1300);
    debts_hm.insert(String::from("home mortgage"), 1500);
    */
}
