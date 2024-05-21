use thiserror::Error;
#[derive(Error, Debug)]
pub enum SimpleError{
    #[error("An error occurred : {0}")]
    GeneralError(String),

    #[error("Invalid Input : {0}")]
    InvalidInput(String),
}

#[derive(Error, Debug)]
pub enum AdvanceError{
    #[error("An Error has occurred: {0}")]
    GeneralError(String),

    #[error("your password must contains only alphabet, {0}")]
    ValidateInput(String),
}
fn processing_value(value: i32)->Result<i32,  SimpleError>{
    if value < 0 {
        return Err(SimpleError::InvalidInput("value must be non-negative".to_string()));
    }
    //simulate some processing
    Ok(value*2)
}
fn password_checker(value: String)->Result<String, AdvanceError>{
 let mut has_no_alphabets = false;
 for c in value.chars(){
    if !((c>='a' && c<='z') || (c >= 'A' && c <= 'Z')){
        
        
        has_no_alphabets = true;
        break;
    }

 }
 if has_no_alphabets{
    return Err(AdvanceError::ValidateInput("have u understood".to_string()));


 }
 Ok(value.to_string())

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
    let mut p =  String::from("12345");
    match password_checker(p){
        Ok(result)=>println!("your saved paassword is okay and approved by deependu jhaaa: {}", result),
        Err(e)=>println!("Error : {}", e),
    }
    p = String::from("deependujha");
     match password_checker(p){
        Ok(result)=>println!("your saved paassword is okay and approved by deependu jhaaa: {}", result),
        Err(e)=>println!("Error : {}", e),
    }
}