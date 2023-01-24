use std::collections::HashMap;

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

// Łańcuchy znaków
//TODO

// DOkumentacja
// TODO

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

// Tekst
fn teksty()
{
	let _t1 = "String\n";
	let t2 = r"surowy string 
	ddd";
	let t3 = r##"tekst z dziwnymi znakami " #"##;
	let _t4 = b"tekst pisany w ASCII";
	println!("{}", t2);
	println!("{}", t3);
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
}
