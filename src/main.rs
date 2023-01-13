fn multiply (vector_1: Vec<String>, vector_2: Vec<String>) -> Vec<String> {
 let mut product: Vec<String> = vec![]       ;
 let mut text   :     String  = String::new();

 for element_1 in vector_1.iter() {
  for element_2 in vector_2.iter() {
   text = element_1.to_owned() + element_2;

   product.push(text);
  }//for element_2 in vector_2.iter() {
 }//for element_1 in vector_1.iter() {

 product
}//fn multiply (vector_1: Vec<String>, vector_2: Vec<String>) -> Vec<String> {

fn variants (text: String) -> Vec<String> {
 let mut chars  : std::str::Chars = text.chars()   ;
 let     collect: Vec<char>       = chars.collect();
 let     length : usize           = collect.len()  ;
 let mut numeric: bool            = true           ;
 let mut variant: Vec<String>     = vec![]         ;

 chars = text.chars();
 for char in chars {
  if !char.is_numeric() {
   numeric = false;
   break          ;
  }//if !char.is_numeric() {
 }//for char in chars {

 if numeric {
  match length {
   0 => {
    //Do nothing ...

   }//0 => {

   1 => {
    chars = text.chars();
    for char in chars {
     match char {
      '2' => { variant.push("a".to_string()); variant.push("b".to_string()); variant.push("c".to_string());                                }
      '3' => { variant.push("d".to_string()); variant.push("e".to_string()); variant.push("f".to_string());                                }
      '4' => { variant.push("g".to_string()); variant.push("h".to_string()); variant.push("i".to_string());                                }
      '5' => { variant.push("j".to_string()); variant.push("k".to_string()); variant.push("l".to_string());                                }
      '6' => { variant.push("m".to_string()); variant.push("n".to_string()); variant.push("o".to_string());                                }
      '7' => { variant.push("p".to_string()); variant.push("q".to_string()); variant.push("r".to_string()); variant.push("s".to_string()); }
      '8' => { variant.push("t".to_string()); variant.push("u".to_string()); variant.push("v".to_string());                                }
      '9' => { variant.push("w".to_string()); variant.push("x".to_string()); variant.push("y".to_string()); variant.push("z".to_string()); }
      _   => {                                                                                                                             }
     }//match char {
    }//for char in chars {
   }//1 => {

   _ => {
    let (left, right) = text.split_at(1);

    variant = multiply(variants(left.to_string()), variants(right.to_string()));
   }//_ => {
  }//match length {
 }//if numeric {

 variant
}//fn variants (text: String) -> Vec<String> {

fn main(){
 loop {
  let mut digits: String = String::new();

  println!("\r\n\r\ndigits:"); 

  std::io::stdin().read_line(&mut digits).expect("Input failed");

  digits = digits.replace("\n", "");
  digits = digits.replace("\r", "");

  if &digits[..] == "exit" {
   break;   

  } else {//if &digits[..] == "exit" {
   let variant: Vec<String> = variants(digits);

   println!("variant: {:?}", variant);
  }//} else {//if &digits[..] == "exit" {
 }//loop {
}//fn main(){
