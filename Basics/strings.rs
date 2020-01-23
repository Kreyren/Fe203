fn main() {
    let mut mystr = String::from("Hello, How it's going?");

    println!("Length:{} and is empty:{}", mystr.len(), mystr.is_empty());

    for token in mystr.split_whitespace() {
        println!("{}", token);
    }
    println!("Does is contain 'uni'? :{}", mystr.contains("uni"));

    mystr.push_str(" test lol");
    print!("{}", mystr);
}
