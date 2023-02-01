use std::collections::HashMap;
use std::fs;
mod moda;
mod modb;

//==================================== Typy ================================================================
// Własne typy
type Tablica = HashMap<u32, String>;
fn tdef_fun(arg: Tablica)
{
	for (val, s) in arg
	{
		println!("{} -> {}", val, s);
	}
}

// Podstawowe typy
fn typy()
{
	let _val1:u32 = 0u32;
	let _val2:usize = 555; // usize, isize - długość zależna od architertury (32/64, przestrzeń adresowa i wskaźniki)
	let _val3 = 1.9f32; // TYp wnioskowany
	let _val4 = 0xABCD_EF01u32; // Podkreślniki zwiększające czytelność (mogą być w dowolnych wartościach)
	println!("{}", 2u16.pow(4));
	
	let mut loc: Tablica = Tablica::new();
	loc.insert(32, "GGttt".to_string());
	loc.insert(11, "fsdfds".to_string());
	tdef_fun(loc);
}

// Referencje
fn ref_f1(_tab: &Tablica){}

fn ref_f2(_tab: &mut Tablica){}

fn ref_fun()
{
	let mut loc: Tablica = Tablica::new();
	ref_f1(&loc); // Referencja współdzielona (tylko do odczytu)
	ref_f2(&mut loc); // Referencja mutowalna
}

//==================================== Struktury ===========================================================
struct Struktura1
{
	var1: u32,
	var2: Vec<u8>
}
//Dziedziczenie metod:
#[derive(Copy, Clone, Debug, PartialEq)]
struct Struktura2(usize, usize); //Struktura z polami numerowanymi

impl Struktura1
{
	pub fn dmet(&self) //Publiczna metoda struktury
	{
		println!("{}, {}", self.var1, self.var2.len());
	}
	pub fn smet() //Statyczna metoda
	{
		println!("smet");
	}
}

fn struktury()
{
	let _str1 = Struktura1{
		var1: 22,
		var2: vec![0; 10]	
	};
	let var1 = 2;
	let var2 = vec![0; 2];
	let str2 = Struktura1{var1, var2};
	let var3 = Struktura2(1, 20);
	println!("{}, {}", var3.0, var3.1);
	str2.dmet();
	Struktura1::smet();
}

//==================================== Enumy ===============================================================

enum En1
{
	En1a,
	En1b,
}

#[derive(Debug)]
enum En2 // Enum z wartościami, coś jak uncja
{
	En2a{aa:u32, bb:u32},
	En2b{aa:f64},
	En2c(f64),
	En2d(f64, f64),
	En2e
}

impl En2
{
	pub fn disp(&self)
	{
		match self
		{
			En2::En2a{aa, bb} => {println!("{aa} {bb}")},
			En2::En2b{aa} => {println!("{aa}")},
			En2::En2c(v) => {println!("{v}")},
			En2::En2d(v, z) => {println!("{v} {z}")},
			En2::En2e => {println!("?")},
		}
	}
}

fn enumy()
{
	let _var1 = En1::En1a;
	let _var2 = En1::En1b;
	let _var3 = En2::En2c;
	let _var4 = En2::En2b{aa:5f64};
	let _var5 = En2::En2a{aa:44, bb:66};
	let _var6 = En2::En2c(6f64);
	let _var7 = En2::En2d(6f64, 55f64);
	let _var8 = En2::En2e;
	println!("{:?} {:?} {:?} {:?}", _var5, _var4, _var6, _var7);
	_var5.disp();
}

//==================================== Funkcje =============================================================
fn funkcja(mut arg:f64) -> f64
{
	assert!(arg>0f64);//Wywołanie makra (wykrzyknik)
	arg = arg.sqrt();
	arg // zwracana wartość (ostatnie wyrażenie bez średnika), można zwracać jawnie: return arg;
}
#[test]
// cargo test
fn test_funkcja()
{
	assert_eq!(funkcja(4f64),2f64);
}

//==================================== Kolekcje ============================================================
// Krotki
fn funkcja_zwracajaca_krotke() -> (u64, f64)
{
	let temp = (666u64, 6.66f64);
	println!("({}, {})", temp.0, temp.1);
	temp
}

fn krotka_u()
{
	let (var1, var2) = funkcja_zwracajaca_krotke();
	println!("({}, {})", var1, var2);
}

// Tablice
fn tablice()
{
	let _tab1:[u64;6] = [1, 2, 3, 4, 5, 6]; //Jawnie określony typ i rozmiar
	let _tab2 = ["asd", "ddd", "dsa"]; //Typ i rozmiar wnioskowany
	let tab3 = [0u8, 32]; //Tablica z 32 elementami wypełniona zerami
	let _tab4 = b"tekst pisany w ASCII"; //Tablica u8
	println!("{}", tab3.len());
}

// Wektory
fn wektory()
{
	let mut _vec1 = Vec::new();
	_vec1.push(44f32);
	_vec1.push(66f32);
	let mut vec2 = vec![5u32, 9u32, 99u32];
	
	for val in &vec2
	{
		println!("{}", val);
	}
	vec2.push(10u32);
}

//==================================== Dokumentacja ========================================================
/// Opis funkcji fun_doc, generowane przez "cargo doc"
fn fun_doc()
{
}

//==================================== Wyrażenia ===========================================================
fn wyrazenia()
{
	let x = false;
	let y = if x {1} else {2}; // if jest wyrażeniem i zwraca wartość
	let mut z = match y // match jest wyrażeniem i zwraca wartość
	{
		1 => 4,
		2 => {println!("fd"); y*2}
		4 => 6,
		_ => 0 // default
	};
	println!("{z}");
	z = 
	{ // blok kodu jest wyrażeniem i zwraca wartość
		let tx = 55;
		let ty = 66;
		tx*ty
	};
	println!("{z}");
	
	for i in 0..5
	{
		println!("for {i}");
	}
}
//==================================== Błędy ===============================================================
fn ret_error() -> Result<String, std::io::Error>
{
	fs::read_to_string("hello.txt")
}

fn ret_some() -> Option<u32>
{
	Some(666u32)
}

fn bledy()
{
	//panic!();
	//panic!("Test panic");
	
	let _v1 = match ret_error()
	{
		Ok(v) => v,
		Err(err) => {
			println!("{}", err);
			println!("{:?}", err);//Dokładniejszy opis pliku
			String::new()
			}
	};
	let _v2 = match ret_some()
	{
		Some(v) => v,
		None => 0
	};
	//let _v3 = ret_error().unwrap();//Zwraca wartość lub panikuje
	//let _v4 = ret_error().expect("text");//j.w. plus wyświetla wiadomość
	//let _v5 = ret_error()?;//Returnuje error jeżeli funkcja zwróci błąd
}

//==================================== Tekst ===============================================================
fn teksty()
{
	let _t1 = "String\n";
	let t2 = r"surowy string 
	ddd";
	let t3 = r##"tekst z dziwnymi znakami " #"##;
	let _t4 = b"tekst pisany w ASCII";
	println!("{}", t2);
	println!("{}", t3);
	wyrazenia();
}

//TODO

//==================================== Moduły ==============================================================
fn mods()
{
	moda::ma_fun();
	modb::mb_fun();
	modb::sub_modb::msb_fun();
}

//==========================================================================================================
fn main() 
{
	println!("{}", funkcja(4f32 as f64)); //Rzutowanie
	typy();
	krotka_u();
	tablice();
	ref_fun();
	wektory();
	teksty();
	bledy();
	fun_doc();
	mods();
	struktury();
	enumy();
}
