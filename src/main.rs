fn main() {
    
    //without explicitly stating the variable type
    let x = 4;

    //explicitly stating the variable type
    let y:i32 = 5;

    println!("x is {} while y is {}", x, y);

    //by default, all variables are immutable
    //so we cannot change their value, once it is assigned

    //to define mutable, use "mut"
    let mut p = 3;

    println!("at first, p is {}", p);

    p = 9;
    println!("but now, p is {}", p);

    //or we can redeclare the variable again
    let p = p + 10;
    println!("and now, p is {}", p);

    //if the variable name is similar but on the different scope, the variable on deeper scope will not affect the outer scope
    let q = 4;
    println!("q outside is {}", q);

    {
        let q = 2;
        println!("q inside is {}", q);
    }

    let q = q + 1;
    println!("q outside after increment is {}", q);

    //if we redefine the variable, we can assign a new type on that variable
    let q = "Bukan, ini huruf";
    println!("Apakah q angka? {}", q);

    //defining a constant is not that different
    const MINUTE_IN_SECONDS:u32 = 60;
    println!("one minute is equals to {} seconds", MINUTE_IN_SECONDS);

    //data types:
    //i -> integer
    let a:i32 = -20;
    //u -> unsigned integer (positive integer)
    let b:u32 = 20;
    //f -> float
    let c:f32 = 20.2;
    //bool -> boolean
    let d:bool = false;
    //char -> character
    let e:char = 'c';

    //tuple (fix length and type sensitives)
    let f:(i32, bool, char) = (1, false, 'd');
    //to print it, we need to print one by one
    println!("The tuple contains, {}, {} and {}", f.0, f.1, f.2);

    //array (fix length and uniform type)
    let g:[i32;5] = [3,6,8,9,2];

    //type casting could be done in several ways
    let aa = 2i8;
    let aa = 2_i32;
    let aa = 20 as i64;

    //to cast string to integer, we need to define the variable type when casting it
    let ss = "3";
    let cc:i32 = ss.trim().parse().unwrap();
    println!("now the string become integer! {}", (cc + 2));

    //condition: [<, >, <=, >=, ==] (only works in same variable type)
    //compound condition: [&&, ||, !]

    if "abc" == "abc" {
        println!("Sama");
    } else {
        println!("Beda");
    }

    coba();
    tambah(10, 20);

    //we can also assign a variable value from an expression
    let po = {
        let pa = 4;
        //if we have a semicolon here, it means this line is a statement. so it won't return anything
        //but without a semicolon, it becomes an expression
        pa + 7
    };
    println!("nilai variable-nya adalah {}", po);
    println!("hasil kurangnya {}", kurang(10, 20));

    //borrowing could also happen in rust
    //borrowing is marked by ampersand
    let mut b = String::from("abc");
    {
        //immutable borrows are allowed for many variable at a time
        let c = &b;
        //while mutable borrows are only allowed one at a time (to avoid multiple variable trying to update a value)
        let d = &mut b;
    }

}

fn coba() {
    println!("Coba");
}

//we need to define variable type in a function arguments
fn tambah(x:i32, y:i32) {
    println!("Totalnya adalah {}", (x+y));
}

//function with a return value
fn kurang(x:i32, y:i32) -> i32 {
    return x-y;
}