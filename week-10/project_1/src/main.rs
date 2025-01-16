struct Laptop{
    model: String,
    price: u32,
    amount: u32
}

impl Laptop{
    fn sell(&mut self, qty: u32)
    {
        self.amount -= qty; 
    }


}
fn display(lapt: &Laptop)
{
    println!("MODEL: {},\nPRICE: {},\nAMOUNT AVAILABLE: {}\n", lapt.model, lapt.price, lapt.amount);
}

fn calc_cost(lapt_dell: & mut Laptop,lapt_hp: &mut Laptop,lapt_ibm: &mut Laptop,lapt_tosh: &mut Laptop,dell_amt: u32, hp_amt: u32, ibm_amt: u32, tosh_amt: u32) -> u32 {
    let cost: u32;
    if lapt_dell.amount >= dell_amt && lapt_hp.amount >= hp_amt && lapt_ibm.amount >= ibm_amt && lapt_tosh.amount >= tosh_amt
    {
        lapt_hp.sell(hp_amt);
        lapt_ibm.sell(ibm_amt);
        lapt_tosh.sell(tosh_amt);
        lapt_dell.sell(dell_amt);
        cost = (lapt_dell.price * dell_amt) + (lapt_hp.price * hp_amt) + (lapt_tosh.price * tosh_amt) + (lapt_ibm.price * ibm_amt);
    }
    else
    {
        println!("INSUFFICIENT AMOUNT IN THE STORE");
        cost = 0;
    }

    return cost;
    
}

fn main()
{
    let mut hp = Laptop{model:String::from("HP"), price: 650000, amount: 10};
    let mut ibm = Laptop{model:String::from("IBM"), price: 755000, amount: 6};
    let mut toshiba = Laptop{model:String::from("TOSHIBA"), price: 550000, amount: 10};
    let mut dell = Laptop{model:String::from("DELL"), price: 850000, amount: 4};
    println!("\n\n    Mr. OGBEIFUNA'S STORE :  ");  
    display(&hp);
    display(&ibm);
    display(&toshiba);
    display(&dell);

    let cost_for_all = calc_cost(&mut dell, &mut hp, &mut ibm, &mut toshiba, 3,3,3,3);
    
    println!("Cost for customer to buy 3 HPs, 3 Toshibas, 3 IBMs and 3 Dells is {}\n            *****TRANSACTION COMPLETED*****", cost_for_all);
}


