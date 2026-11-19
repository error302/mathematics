fn main() {
    let artifacts: Vec<_> = ["number","add","subtract","five","five_sub","ten","ten_sub","compound","mental"].iter().map(|stage|axiom_math::generate(&format!("abacus.trainer.{stage}"),"1.1.0",&"0".repeat(64),1,&"0".repeat(64)).unwrap()).collect();
    println!("{}", serde_json::to_string(&artifacts).unwrap());
}
