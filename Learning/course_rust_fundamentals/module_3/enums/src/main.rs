#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    Vaud,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"), // only Rioja is supported
        _ => println!("{:?} is not supported!", w),            // match catch-all
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Nyon Vieilles"),
        region: WineRegions::Vaud,
    };

    println!("Wine 1: {} is from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} is from {:?}", wine2.name, wine2.region);
    println!("Wine 2: {} is from {:?}", wine3.name, wine3.region);
    supported_regions(wine1.region);
    supported_regions(WineRegions::Rioja);
}
