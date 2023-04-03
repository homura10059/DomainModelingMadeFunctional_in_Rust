// Type Signature
fn add1(x: isize, y: isize) -> isize {
    x + y
}

fn add(x: isize, y: isize) -> impl Fn(isize, isize) -> isize {
    move |x, y| x + y
}

// Functions with Generic Types
fn square_plus_one(x: isize) -> isize {
    let square = move |x| x * x;
    square(x) + 1
}

fn are_equal<T: PartialEq>(x: T, y: T) -> bool {
    x == y
}

// Composition of Types

struct AppleFruit {}
struct BananaFruit {}
struct CherriesFruit {}

// and type
struct FruitSnack {
    apple: AppleFruit,
    banana: BananaFruit,
    cherries: CherriesFruit,
}

// or type
enum FruitSnackEnum {
    Apple(AppleFruit),
    Banana(BananaFruit),
    Cherries(CherriesFruit),
}

// Simple Types
type Kilometers = i32;
