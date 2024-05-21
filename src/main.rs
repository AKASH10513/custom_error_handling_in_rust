use thiserror::Error;
#[derive(Error, Debug)]
pub enum SimpleError{
    #[error("An error occurred : {0}")]
    GeneralError(String),

    #[error("Invalid Input : {0}")]
    InvalidInput(String),
}
fn processing_value(value: i32)->Result<i32,  SimpleError>{
    if value < 0 {
        return Err(SimpleError::InvalidInput("value must be non-negative".to_string()));
    }
    //simulate some processing
    Ok(value*2)
}
fn main() {
    match processing_value(-10){
        Ok(result)=>println!("processed value: {}", result),
        Err(e) =>println!("Error: {}",e),

    }
    match processing_value(12){
        Ok(result)=>println!("processed value : {}", result),
        Err(e)=>println!("Error : {}",e),
    }
}