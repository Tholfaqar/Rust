
pub fn calc_cashflow(assets: Vec<i32>, liabilities: Vec<i32>)
{
    let mut asset_total = 0;
    let mut liability_total = 0;

    for asset in &assets
    {
        asset_total += asset;
    }

    for liability in &liabilities
    {
        liability_total += liability;
    }
    
    let cash = asset_total - liability_total;
    println!("total cash flow: {}", cash);
}
/*
pub fn calc_debt_income_ratio (assets: HashMap, liabilities: HashMap) {

    let mut dtr: f32 = 0.00;
    println!("DTR: {}", dtr);
}
*/

