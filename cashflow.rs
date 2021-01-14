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