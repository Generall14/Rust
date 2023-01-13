extern crate iron; // paczki zewnętrzne
extern crate router;
extern crate urlencoded;
#[macro_use] extern crate mime; // makra z paczki mime

use std::io::Write;
use std::str::FromStr;
use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;

fn gcd(mut n: u64, mut m: u64) -> u64
{
	assert!(n!=0 && m!=0);//Wywołanie makra (wykrzyknik)
	while m!=0
	{
		if m<n
		{
			let t=m;//zmienna lokalna, typ wywnioskowany, można podać let t: u64 = m;
			m=n;
			n=t;
		}
		m=m%n;
	}
	n // zwracana wartość (ostatnie wyrażenie bez średnika), można zwracać jawnie: return n;
}

#[test]
fn test_gcd()
{
	assert_eq!(gcd(14,15),1);
	assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}

fn main()
{
	let mut numbers = Vec::new();//Wektor, typ wywnioskowany, można podać Vec<u64>
	
	for arg in std::env::args().skip(1)
	{
		numbers.push(u64::from_str(&arg).expect("Błąd parsowania argumentu"));//Zwraca typ Result
	}
	
	if numbers.len() == 0
	{
		let mut router = Router::new();
		router.get("/", get_form, "root");
		router.post("/gcd", post_gcd, "gcd");
		
		writeln!(std::io::stderr(), "Brak argumentów").unwrap();
		//std::process::exit(1);
		println!("Serwer dostępny pod adresem http://localhost:3000");
		Iron::new(router).http("localhost:3000").unwrap();
	}
	else
	{
		let mut d = numbers[0];
		for m in &numbers[1..] // Iterator jako referencje
		{
			d = gcd(d, *m);
		}
		println!("największy wspólny dzielnik {:?} to {}", numbers, d);
	}
	
    println!("Hello, world!");
}

fn get_form(_request: &mut Request) -> IronResult<Response> // nazwa argumentu z podkreślnikiem - brak warningu o nieużywanum argumencie
{
	let mut response = Response::new();
	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut(r#"
	<title>Kalkulator GCD</title>
	<form action="/gcd" method="post">
	  <input type="text" name="n"/>
	  <input type="text" name="n"/>
	  <button type="submit">Oblicz GCD</button>
	</form>
	"#);// surowy łańcuch znaków
	
	Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response>
{
	let mut response = Response::new();
	let form_data = match request.get_ref::<UrlEncodedBody>()
	{
		Err(e) =>
		{
			response.set_mut(status::BadRequest);
			response.set_mut(format!("Błąd paarsowania danych formularza {:?}\n", e));
			return Ok(response);
		}
		Ok(map) => map
	};
	
	let unparset_numbers = match form_data.get("n")
	{
		None =>
		{
			response.set_mut(status::BadRequest);
			response.set_mut(format!("Formularz nie zawiera parametru n\n"));
			return Ok(response);
		}
		Some(nums) => nums
	};
	
	let mut numbers = Vec::new();
	for unparsed in unparset_numbers
	{
		match u64::from_str(&unparsed)
		{
			Err(_) =>
			{
				response.set_mut(status::BadRequest);
				response.set_mut(format!("Wartość parametru 'n' nie jest liczbą: {:?}\n", unparsed));
				return Ok(response);
			}
			Ok(n) => {numbers.push(n);}
		}
	}
	
	let mut d = numbers[0];
	for m in &numbers[1..]
	{
		d = gcd(d, *m);
	}
	
	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut(format!("Największy wspólny dzielnik liczb {:?} wynosi <b>{}</b>\n", numbers, d));
	
	Ok(response)
}