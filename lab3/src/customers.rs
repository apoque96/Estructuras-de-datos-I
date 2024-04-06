#[derive(Debug, Clone, Copy)]
pub struct CustomerOfProperty<'a>{
    pub dpi: u64,
    pub budget: u32,
    pub date: &'a str,
}

pub fn create_customer_of_property(dpi: u64, budget: u32, date: &str) -> CustomerOfProperty{
    CustomerOfProperty{
        dpi,
        budget,
        date,
    }
}