use std::io::Write;

fn main() {

    let commissioner_name = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zone = vec!["South West", "North East", "South South", "South West", "South East"];

    let mut file = std::fs::File::create("convicted.ministers.txt").expect("failed to create");

    file.write_all("                            CONVICTED MINISTERS".as_bytes()).expect("failed to write");
    file.write_all("\n\n        NAME OF COMMISIONER                              MINISTRY                               GEOPOLITICAL ZONE\n".as_bytes()).expect("failed to write");
    for i in 0..zone.len()
    {
        let index = i as usize;
        file.write_all(commissioner_name[index].as_bytes()).expect("failed to write");
        file.write_all("                                    ".as_bytes()).expect("failed to write");
        file.write_all(ministry[index].as_bytes()).expect("failed to write");
        file.write_all("                                    ".as_bytes()).expect("failed to write");
        file.write_all(zone[index].as_bytes()).expect("failed to write");
        file.write_all("\n".as_bytes()).expect("failed to write");
    }
}