mod graf;

fn main() {
    let g1 = graf::Graf::nacitaj_subor("grafy/G1.hrn");
    
    println!("{:?}", g1.vypis_graf());
}
