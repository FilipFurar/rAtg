use std::fs; //file handling

pub(crate) struct Graf {
    n: u32, // pocet vrcholov v grafe
    m: u32, // pocet hran v grafe
    edges: Vec<Hrana>, // hrany grafu
}

struct Hrana {
    u: u32, // prvy vrchol hrany
    v: u32, // druhy vrchol hrany
    c: u32, // cena hrany
}

impl Graf {
    pub fn new(n: u32, m: u32) -> Graf { //priprava na tvorbu noveho grafu keby ho nenacitavame zo suboru
        Graf {
            n,
            m,
            edges: Vec::new(),
        }
    }

    /*
        Nacitanie grafu zo suboru:
        Na kazdom riadku su tri cisla, prve a druhe cislo su cisla vrcholov
        a tretie cislo je ohodnotenie hrany.
        Pocet vrcholov aj pocet hran sa urci automaticky. Pocet hran je rovny
        poctu riadkov v subore a pocet vrcholov je rovny najvacsiemu cislu
        vrcholu v udajoch o hranach.
    */
    pub(crate) fn nacitaj_subor(cesta_ku_suboru: &str) -> Graf {
        let data = fs::read_to_string(cesta_ku_suboru).expect("Chyba pri nacitani suboru");

        let mut n = 1; //pocet vrcholov
        let mut m = 0; //pocet hran

        let mut edges = Vec::new();

        for line in data.lines() {
            let vrcholy: Vec<&str> = line.split_whitespace().collect(); //oddelime cisla pomocou medzier a novych riadkov, po jednom hadzeme do vectora
                                                                        //&str je string slice reference, https://doc.rust-lang.org/book/ch04-03-slices.html?highlight=string#string-slices
            let u: u32 = vrcholy[0].parse().unwrap(); //nulty (prvy) vrchol
            let v: u32 = vrcholy[1].parse().unwrap(); //druhy vrchol
            let c: u32 = vrcholy[2].parse().unwrap(); //cena hrany

            let h = Hrana { u, v, c };
            m += 1; //pocet hran sa zvysi o 1
            if u > n {
                n = u; //nastavime novy pocet vrcholov, ak najdem vacsie cislo
            }
            if v > n {
                n = v; //nastavime novy pocet vrcholov, ak najdem vacsie cislo
            }
            
            edges.push(h);
        }
        
        Graf {
            n,
            m,
            edges,
        } //vrati novy graf
    }
    
    pub fn vypis_graf(&self) {
        println!("Pocet vrcholov: {}", self.n);
        println!("Pocet hran: {}", self.m);
        println!("Hrany:");
        for h in &self.edges {
            println!("{} {} {}", h.u, h.v, h.c);
        }
    }
}