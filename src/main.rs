#[derive(Debug)]
pub enum Sexe {
    H,
    F,
}

use std::error::Error;
use postgres::types::FromSql;
use postgres::types::Type;
use postgres::{Client, NoTls};

impl FromSql<'_> for Sexe {
    fn from_sql(
	_sql_type: &Type,
	value: &[u8]
    ) -> Result<Self, Box<dyn Error + Sync + Send>> {
	match value {
	    b"H" => Ok(Sexe::H),
	    b"F" => Ok(Sexe::F),
	    _ => Ok(Sexe::H),
	}
    }
    fn accepts(sql_type: &Type) -> bool {
	sql_type.name() == "sexe"
    }
}

fn test2(st: &str) {
    println!("{}",st);
    let mut client = Client::connect(st, NoTls).unwrap();

    for row in client.query("SELECT nom,prenom,sexe,annee_n,mois_n,jour_n,insee_n,commune_n,pays_n,annee_d,mois_d,jour_d,insee_d,num_acte FROM dc where nom='MARTIN' and prenom ~* '^NICOLAS'", &[]).unwrap() {
//	let id: i32 = row.get(0);
	let nom: &str = row.get(0);
	let prenom: &str = row.get(1);
	let sexe: Sexe = row.get(2);
	let annee_n:i16 =row.get(3);
  	let mois_n:i16 =row.get(4);
	let jour_n:i16 =row.get(5);
  	let insee_n: &str =row.get(6);
  	let commune_n:&str =row.get(7);
  	let pays_n:&str =row.get(8);
  	let annee_d:i16 =row.get(9);
  	let mois_d:i16 =row.get(10);
	let jour_d:i16 =row.get(11);
  	let insee_d:  &str =row.get(12);
  	let num_acte: &str =row.get(13);
	println!(
	    "found person: {} {} {:?} {} {} {} {} {} {} {} {} {} {} {} ",
	    nom,prenom,sexe,annee_n,mois_n,jour_n,insee_n,commune_n,pays_n,annee_d,mois_d,jour_d,insee_d,num_acte);
    }
}

use argparse::{ArgumentParser, Store};
fn main() {
    let mut hostaddr = "192.168.1.1".to_string();
    let mut user = "alliot".to_string();
    let mut password = "".to_string();
    let mut dbname = "dc".to_string();

    { // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Finding dead people");
        ap.refer(&mut hostaddr)
            .add_option(&["-h","--hostaddr"], Store,"Hostaddr (default 192.168.1.1)");
        ap.refer(&mut user)
            .add_option(&["-u","--user"], Store,"User (default alliot)");
        ap.refer(&mut password)
            .add_option(&["-p","--password"], Store,"Password (default '')");
        ap.refer(&mut dbname)
            .add_option(&["-d","--dbname"], Store,"Dbname (default dc)");
        ap.parse_args_or_exit();
    }
    let st =
        "hostaddr=".to_owned()+&hostaddr+" user="+&user+
        " password="+&password+" dbname="+&dbname;
    println!("{}",st);
    test2(&st);
}
