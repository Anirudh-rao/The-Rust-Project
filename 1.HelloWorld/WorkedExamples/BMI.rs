// This module provides body measurement and health metrics calculation tools
// !
// ! Its includes calculation BMI which is standard
// ! Measure of body fat based on height and weight


// Calculate the Body mass index(BMI) of a person
// Formula
// BMI is calculated as: `weight_kg / (height_m * height_m)`

pub fn calcualte_bmi(weight_kg:f64, height_m: f64) -> f64{
    weight_kg / (height_m * height_m)
}

fn main(){
    let BMI = calcualte_bmi(70.0, 1.75);
    println!("BMI :{:.2}",BMI);
}