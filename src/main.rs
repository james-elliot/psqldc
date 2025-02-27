#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize,serde::Serialize)]
#[allow(dead_code)]
 struct DataCsv  {
     ICCA_stay_id:Option<String>,
     IEP:Option<String>,
     IPP:Option<String>,
     Nom:Option<String>,
     Prenom:Option<String>,
     Date_de_naissance:Option<String>,
     Date_entree:Option<String>,
     Motif_admission:Option<String>,
     Conclusion:Option<String>,
     Décè:Option<String>,
 }

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use std::cmp::min;
fn read_write_csv(mut client: Client,path: &str)  {
//    let matcher = SkimMatcherV2::default();
    let file = std::fs::File::open(path).unwrap();
    let mut reader = csv::ReaderBuilder::new().delimiter(b',').has_headers(true).from_reader(file);
    let output_file = std::fs::File::create("output.csv").unwrap();
    let mut writer = csv::WriterBuilder::new().delimiter(b',').has_headers(true).from_writer(output_file);
    for result in reader.deserialize::<DataCsv>() {
        let mut r = result.unwrap();
	let nom = r.Nom.clone().unwrap();
	let prenom = r.Prenom.clone().unwrap();
	let prenoms: Vec<&str> = prenom.split(&['-', ' ', ','][..]).collect();
	let date = r.Date_de_naissance.clone().unwrap();
	let (date_part,_time_part) = date.split_once('T').unwrap();
	let parts: Vec<&str> = date_part.split('-').collect();
	let (annee, mois, jour) = (parts[0], parts[1], parts[2]);
	let v_jour:i16=jour.parse().unwrap();
	let v_mois:i16=mois.parse().unwrap();
	let v_annee:i16=annee.parse().unwrap();
//	let deces = if let Some(x)=r.Décè.clone() {if x=="true" {println!("grrr");true} else{false}} else {false};
	let deces = if let Some(x)=r.Décè.clone() {x=="true"} else {false};
	r.Décè=None;
	let mut best_score = 0.5;
	//	for row in client.query("SELECT prenom,annee_d,mois_d,jour_d FROM dc where jour_n=$1 and mois_n=$2 and annee_n=$3 and nom=$4 and prenom ~* $5 ", &[&v_jour,&v_mois,&v_annee,&nom,&prenom]).unwrap() {
	//	for row in client.query("SELECT prenom,annee_d,mois_d,jour_d FROM dc where jour_n=$1 and mois_n=$2 and annee_n=$3 and nom=$4 and prenom like $5", &[&v_jour,&v_mois,&v_annee,&nom,&(prenom.clone()+"%")]).unwrap() {
	for row in client.query("SELECT prenom,annee_d,mois_d,jour_d FROM dc where jour_n=$1 and mois_n=$2 and annee_n=$3 and nom=$4", &[&v_jour,&v_mois,&v_annee,&nom]).unwrap() {
	    let prenom_complet: &str = row.get(0);
	    let prenoms_complet: Vec<&str> = prenom_complet.split(&['-', ' ', ','][..]).collect();
	    let dim = min(3,min(prenoms_complet.len(),prenoms.len()));
	    let (mut tscore,mut coefs) = (0.,0);
	    for i in 0..dim {
		coefs += dim-i;
		tscore += ((dim-i) as f32)*rust_fuzzy_search::fuzzy_compare(&prenoms_complet[i], &prenoms[i]);
	    }
	    tscore /=  coefs as f32;
//	    let _score = matcher.fuzzy_match(&prenom_complet, &prenom);
//	    let _score2 = levenshtein::levenshtein(&prenom_complet, &prenom);
//	    println!("{} {} : {} {} {} {}",prenom,prenom_complet,vscore,score2,score3,tscore);
	    if tscore >= best_score{
  		let annee_d: i16 = row.get(1);
  		let mois_d: i16 = row.get(2);
		let jour_d: i16 = row.get(3);
		let date = jour_d.to_string()+"/"+&mois_d.to_string()+"/"+&annee_d.to_string();
		r.Décè=Some(date.clone());
		best_score=tscore;
		if tscore!=1.0 {println!("{}, {}, {} : {}, {}",prenom,prenom_complet,nom,date,tscore)};
	    }
	}
	if deces && r.Décè.is_none() {println!("Zorglub:{} {} {}",prenom,nom,date)};
	writer.serialize(&r).unwrap();
    }
}

use time::Date;
//use std::collections::HashMap;
use multimap::MultiMap;
fn test_names(mut client: Client)  {
    let org_year = 1946;
    let (mut snb,mut sdup,mut sdup2,mut sdup3,mut sdup4) = (0,0,0,0,0);
    let mut date = Date::from_ordinal_date(org_year,1).unwrap();
    for _i in 1..366 {
	let mut names = MultiMap::new();
	let year = Date::year(date) as i16;
	if year!=org_year as i16 {break};
	let month = Date::month(date) as i16;
	let day = Date::day(date) as i16;
	let res = client.query("SELECT prenom,nom,insee_n,annee_d,mois_d,jour_d,insee_d FROM dc WHERE jour_n=$1 AND mois_n=$2 AND annee_n=$3",
			       &[&day,&month,&year]).unwrap();
	let nb = res.len();
	for row in res {
	    let prenom_complet:&str = row.get(0);
	    let nom:&str = row.get(1);
	    let insee_n:&str = row.get(2);
	    let yd:i16 = row.get(3);
	    let md:i16 = row.get(4);
	    let dd:i16 =  row.get(5);
	    let insee_d:&str = row.get(6);
	    let prenoms: Vec<&str> = prenom_complet.split(&['-', ' ', ','][..]).collect();
	    names.insert(nom.to_string(),(prenoms[0].to_string(),insee_n.to_string(),yd,md,dd,insee_d.to_string()));
	}
	let (mut dup,mut dup2,mut dup3,mut dup4)=(0,0,0,0);
	for (key,values) in names.iter_all() {
	    let mut v = values.clone();
	    v.sort();
	    for i in 0..v.len()-1 {
		if v[i].0==v[i+1].0 {
//		    println!("{} - {} {} {} {} {}- {} {} {} {} {}",key,v[i].0,v[i].1,v[i].2,v[i].3,v[i].4,v[i+1].0,v[i+1].1,v[i+1].2,v[i+1].3,v[i+1].4);
		    dup+=1;
		    if v[i].1==v[i+1].1 {dup2+=1;}
		    if (v[i].2==v[i+1].2)&&(v[i].3==v[i+1].3)&&(v[i].4==v[i+1].4) {dup3+=1}
		    if v[i].5==v[i+1].5 {dup4+=1;}
		};
	    }
	}
	println!("{} {} {} {} {} {}",date,nb,dup,dup2,dup3,dup4);
	snb+=nb;sdup+=dup;sdup2+=dup2;sdup3+=dup3;sdup4+=dup4;
	date=date.next_day().unwrap();
    }
    println!("{} {} {} {} {} {}",org_year,snb,sdup,sdup2,sdup3,sdup4);
}
//1966 75502 536 498 498 486
//1956 156731 1071 938 896 889
//1946 274466 2099 1692 1641 1597
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

<<<<<<< HEAD
=======
fn test2(client: &mut Client,nom: &str,prenom: &str) {
//    for row in client.query("SELECT nom,prenom,sexe,annee_n,mois_n,jour_n,insee_n,commune_n,pays_n,annee_d,mois_d,jour_d,insee_d,num_acte FROM dc where nom='MARTIN' and prenom ~* '^NICOLAS'", &[]).unwrap() {
    for row in client.query("SELECT nom,prenom,sexe,annee_n,mois_n,jour_n,insee_n,commune_n,pays_n,annee_d,mois_d,jour_d,insee_d,num_acte FROM dc where nom=$1 and prenom ~* $2", &[&nom,&prenom]).unwrap() {
//	let id: i32 = row.get(0);
	let nom: &str = row.get(0);
	let prenom: &str = row.get(1);
	let sexe: Sexe = row.get(2);
	let annee_n: i16 = row.get(3);
  	let mois_n: i16 = row.get(4);
	let jour_n: i16 = row.get(5);
  	let insee_n: &str = row.get(6);
  	let commune_n: &str = row.get(7);
  	let pays_n: &str = row.get(8);
  	let annee_d: i16 = row.get(9);
  	let mois_d: i16 = row.get(10);
	let jour_d: i16 = row.get(11);
  	let insee_d: &str = row.get(12);
  	let num_acte: &str = row.get(13);
	println!(
	    "found person: {} {} {:?} {} {} {} {} {} {} {} {} {} {} {} ",
	    nom,prenom,sexe,annee_n,mois_n,jour_n,insee_n,commune_n,pays_n,annee_d,mois_d,jour_d,insee_d,num_acte);
    }
}

>>>>>>> 94a0a91feea4629efab619bcf863b8fa33954af5
use argparse::{ArgumentParser, Store};
use rpassword::read_password;
fn main() {
    let mut hostaddr = "192.168.1.1".to_string();
    let mut dbname = "dc".to_string();
    let mut user = "alliot".to_string();
    let mut password = "".to_string();
//    let mut name = "MARTIN".to_string();
//    let mut surname = "".to_string();
    
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

    if  password=="" {
	println!("Please enter password:");
	password = read_password().expect("Failed to read input");
    }
    
    let st =
        "hostaddr=".to_owned()+&hostaddr+" user="+&user+
        " password="+&password+" dbname="+&dbname;
<<<<<<< HEAD
    let client = Client::connect(&st, NoTls).unwrap();
    test_names(client);
//    read_write_csv(client,"/mnt/c/Users/alliot/Downloads/patients_2024.csv");
=======
    let mut client = Client::connect(&st, NoTls).unwrap();
    test2(&mut client,&name,&surname);
>>>>>>> 94a0a91feea4629efab619bcf863b8fa33954af5
}
