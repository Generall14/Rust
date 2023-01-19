// Podsstawowe typy
fn typy()
{
	let _val1:u32 = 0u32;
	let _val2:usize = 555; // usize, isize - długość zależna od architertury (32/64, przestrzeń adresowa i wskaźniki)
	let _val3 = 1.9f32; // TYp wnioskowany
	let _val4 = 0xABCD_EF01u32; // Podkreślniki zwiększające czytelność (mogą być w dowolnych wartościach)
	println!("{}", 2u16.pow(4));
}

// Funkcje
fn funkcja(mut arg:f64) -> f64
{
	assert!(arg>0f64);//Wywołanie makra (wykrzyknik)
	arg = arg.sqrt();
	arg // zwracana wartość (ostatnie wyrażenie bez średnika), można zwracać jawnie: return arg;
}
#[test]
fn test_funkcja()
{
	assert_eq!(funkcja(4f64),2f64);
}

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
	println!("{}", tab3.len());
}

fn main() 
{
	println!("{}", funkcja(4f32 as f64)); //Rzutowanie
	typy();
	krotka_u();
	tablice();
    println!("Hello, world!");
}
