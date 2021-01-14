mod cashflow;

pub fn main(){
    let mut asset: Vec<i32> =Vec::new(); 
    let mut liab: Vec<i32> = Vec::new();

    asset.push(20);
    asset.push(30);

    liab.push(5);
    liab.push(10);

    cashflow::calc_cashflow(asset, liab);
}
