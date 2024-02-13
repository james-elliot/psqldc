use postgres::{Client, NoTls};


fn test() {
    let mut client = Client::connect("hostaddr=192.168.1.1 user=alliot password= dbname=alliot", NoTls).unwrap();
    
    client.batch_execute("
    CREATE TABLE person (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL,
        data    BYTEA
    )
").unwrap();
    
    let name = "Ferris";
    let data = None::<&[u8]>;
    client.execute(
	"INSERT INTO person (name, data) VALUES ($1, $2)",
	&[&name, &data],
    ).unwrap();
    
    for row in client.query("SELECT id, name, data FROM person", &[]).unwrap() {
	let id: i32 = row.get(0);
	let name: &str = row.get(1);
	let data: Option<&[u8]> = row.get(2);
	
	println!("found person: {} {} {:?}", id, name, data);
    }
}

#[derive(Debug)]
pub enum Sexe {
    H,
    F,
}
use std::error::Error;
use postgres::types::FromSql;
use postgres::types::Type;
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

fn test2() {
    let mut client = Client::connect("hostaddr=192.168.1.1 user=alliot password= dbname=dc", NoTls).unwrap();
    
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

fn main() {
    test2();
}
