use std::f64::consts::PI;
use std::{fmt, mem};

use std::fmt::{DebugList, Formatter, write, Display};
use std::os::unix::raw::ino_t;


// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};
fn main1() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 100);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "lihe", "yuanjie");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 20);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:<width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:0>width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // done ^ Add the missing argument: "James"

    // Create a structure named `Structure` which contains an `i32`.
    // #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.

    let pi = PI;

    println!("{number:.2}", number=pi);


    struct Unprintable(i32);

    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);

}
fn main1_2_2(){
    struct Structure(i32);

    impl fmt::Display for Structure {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug)]
    struct MinMax(i64, i64);

    // Implement `Display` for `MinMax`.
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    // Define a structure where the fields are nameable for comparison.
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    // Similarly, implement `Display` for `Point2D`
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);
    // fixme

    struct Complex {
        real: i32,
        imag: i32
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "read : {}, imag {}", self.real, self.imag)
        }
    }
    let complex = Complex{real:5, imag: 10};
    println!("complex {}", complex)

}
fn main1_2_2_1(){
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;

            for (i, m) in vec.iter().enumerate() {
                if i != 0 { write!(f, ",")?; };
                write!(f, "{} : {}" , i , m)?;
                // todo ? meaning
            }

            write!(f, "]")
        }
    }
    let v = List(vec![1,2,3]);
    print!("{}", v);
}
fn main1_2_3() {
    struct City {
        name: &'static str,
        // Latitude
        lat: f32,
        // Longitude
        lon: f32,
    }

    impl Display for City {
        // `f` is a buffer, and this method must write the formatted string into it
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            // `write!` is like `format!`, but it will write the formatted string
            // into a buffer (the first argument)
            write!(f, "{}: {:.3}°{} {:.3}°{}",
                   self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "RGB ({} {} {}) ", self.red, self.green, self.blue)?;
            write!(f, "0x{:0<2X}{:0<2X}{:0<2X}", self.red, self.green, self.blue)

        }
    }

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", *color);
    }
}

fn main2() {
    // 变量可以给出类型说明。
    let logical: bool = true;

    let a_float: f64 = 1.0;  // 常规说明
    let an_integer   = 5i32; // 后缀说明

    // 否则会按默认方式决定类型。
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // 类型也可根据上下文自动推断。
    let mut inferred_type = 12; // 根据下一行的赋值推断为 i64 类型
    inferred_type = 4294967296i64;

    // 可变的（mutable）变量，其值可以改变。
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // 报错！变量的类型并不能改变。
    // mutable = true;

    // 但可以用遮蔽（shadow）来覆盖前面的变量。
    let mutable = true;
}
fn main2_1() {
    // 整数相加
    println!("1 + 2 = {}", 1u32 + 2);

    // 整数相减
    println!("1 - 2 = {}", 1i32 - 2);
    // 试一试 ^ 尝试将 `1i32` 改为 `1u32`，体会为什么类型声明这么重要

    // 短路求值的布尔逻辑
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 位运算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 使用下划线改善数字的可读性！
    println!("One million is written as {}", 1_000_000u32);
}
fn main2_2() {
    // 元组可以充当函数的参数和返回值
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        // 可以使用 `let` 把一个元组的成员绑定到一些变量
        let (integer, boolean) = pair;

        (boolean, integer)
    }

    // 在 “动手试一试” 的练习中要用到下面这个结构体。
    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    // 包含各种不同类型的元组
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // 通过元组的下标来访问具体的值
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 元组也可以充当元组的元素
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 元组可以打印
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 但很长的元组无法打印
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // 试一试 ^ 取消上面两行的注释，阅读编译器给出的错误信息。
    // todo why?

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 元组可以被解构（deconstruct），从而将值绑定给变量
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    impl Display for Matrix {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
        }
    }
    println!("{}", matrix);
    println!("{:?}", matrix);

    fn transpose(mut matrix: Matrix) -> Matrix {
        let tmp = matrix.0;
        matrix.0 = matrix.3;
        matrix.3 = tmp;
        // let res = Matrix(matrix.3, matrix.1, matrix.3, matrix.0);
        // println!("res : {}", res);
        matrix
    }

    println!("{}", transpose(matrix));
}
fn main2_3() {
    // 此函数借用一个 slice
    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    // 定长数组（类型标记是多余的）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let xs1 = [5, 4, 3, 2, 1];

    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];
    let ys1 = [0, 500];

    // 下标从 0 开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在栈中分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以自动被借用成为 slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slice 可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // 越界的下标会引发致命错误（panic）
    // println!("{}", xs[5]);
}

fn main3_1() {

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8
    }
    // 单元结构体
    struct Unit;

    // 元组结构体
    struct Pair(i32, f32);

    // 带有两个字段的结构体
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    // 结构体可以作为另一个结构体的字段
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Rectangle {
        // 可以在空间中给定左上角和右下角在空间中的位置来指定矩形。
        top_left: Point,
        bottom_right: Point,
    }

    // 使用简单的写法初始化字段，并创建结构体
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // 以 Debug 方式打印结构体
    println!("{:?}", peter);

    // 实例化结构体 `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用结构体更新语法创建新的 point，
    // 这样可以用到之前的 point 的字段
    let bottom_right = Point { x: 5.2, y: 5.3 };
    // ..point? 缺省的就一样

    // `new_point.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 使用 `let` 绑定来解构 point
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // 实例化一个单元结构体
    let _unit = Unit;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    fn abs(mut num: f32) -> f32 {
        if num < 0.0f32 {
            num = -num;
        }
        num

    }

    fn rect_area(rect: Rectangle) -> f32 {
        let Rectangle{ top_left, bottom_right }: Rectangle = rect;
        abs(top_left.x - bottom_right.x) * abs(top_left.y - bottom_right.y)
    }

    fn square(point: Point, len: f32) -> Rectangle {
        let x = point.x;
        let y = point.y;
        Rectangle {
            top_left: point,
            bottom_right: Point {
                x: x + len,
                y: y + len
            }
        }
    }

    unsafe { println!("test rect_area: {}", rect_area(_rectangle)); }
    println!("test square :{:?}", square(point, 10.0));

}


fn main3_2() {
    #![allow(dead_code)]
    // 创建一个 `enum`（枚举）来对 web 事件分类。注意变量名和类型共同指定了 `enum`
    // 取值的种类：`PageLoad` 不等于 `PageUnload`，`KeyPress(char)` 不等于
    // `Paste(String)`。各个取值不同，互相独立。
    enum WebEvent {
        // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
        PageLoad,
        PageUnload,
        // 或者一个元组结构体，
        KeyPress(char),
        Paste(String),
        // 或者一个普通的结构体。
        Click { x: i64, y: i64 }
    }

    // 此函数将一个 `WebEvent` enum 作为参数，无返回值。
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            // 从 `enum` 里解构出 `c`。
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // 把 `Click` 解构给 `x` and `y`。
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            },
        }
    }

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    // Creates a type alias
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;

    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    println!("{}", x.run(1, 5));

}
/*
fn main3_2_1() {
    enum Status {
        Rich,
        Poor,
    }

    enum Work {
        Civilian,
        Soldier,
    }
    // 显式地 `use` 各个名称使他们直接可用，而不需要指定它们来自 `Status`。
    // use Status::{Poor, Rich};
    // 自动地 `use` `Work` 内部的各个名称。
    // use Work::*;

    // `Poor` 等价于 `Status::Poor`。
    let status = Poor;
    // `Civilian` 等价于 `Work::Civilian`。
    let work = Civilian;

    match status {
        // 注意这里没有用完整路径，因为上面显式地使用了 `use`。
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // 再次注意到没有用完整路径。
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}
 */
use List::*;
fn main3_2_2() {
    // 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
    enum Number {
        Zero,
        One,
        Two,
    }

    // 拥有显式辨别值（explicit discriminator）的 enum
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }
    // `enum` 可以转成整型。
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
enum List {
    // Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
    Cons(u32, Box<List>),
    // Nil：末结点，表明链表结束
    Nil,
}

// 可以为 enum 定义方法
impl List {
    // 创建一个空的 List 实例
    fn new() -> List {
        // `Nil` 为 `List` 类型（译注：因 `Nil` 的完整名称是 `List::Nil`）
        Nil
    }

    // 处理一个 List，在其头部插入新元素，并返回该 List
    fn prepend(self, elem: u32) -> List {
        // `Cons` 同样为 List 类型
        Cons(elem, Box::new(self))
    }

    // 返回 List 的长度
    fn len(&self) -> u32 {
        // 必须对 `self` 进行匹配（match），因为这个方法的行为取决于 `self` 的
        // 取值种类。
        // `self` 为 `&List` 类型，`*self` 为 `List` 类型，匹配一个具体的 `T`
        // 类型要好过匹配引用 `&T`。
        match *self {
            // 不能得到 tail 的所有权，因为 `self` 是借用的；
            // 因此使用一个对 tail 的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // （递归的）基准情形（base case）：一个长度为 0 的空列表
            Nil => 0
        }
    }

    // 返回列表的字符串表示（该字符串是堆分配的）
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，
                // 而不是打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}
fn main3_2_3() {


    // 创建一个空链表
    let mut list = List::new();

    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

fn main3_3() {
    // 全局变量是在在所有其他作用域之外声明的。
    static LANGUAGE: &'static str = "Rust";
    static LANGUAGE1: &str = "Rust1";
    const  THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        // 在一般函数中访问常量
        n > THRESHOLD
    }

    let n = 16;

    // 在 main 函数（主函数）中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 报错！不能修改一个 `const` 常量。
    // THRESHOLD = 5;
    // 改正 ^ 注释掉此行
}
fn main4() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 将 `an_integer` 复制到 `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用的变量绑定产生警告；可以给变量名加上下划线前缀来消除警告。
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
    // 改正 ^ 在变量名前加上下划线以消除警告
}

fn main4_1() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // 正确代码
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // 错误！
    // _immutable_binding += 1;
    // 改正 ^ 将此行注释掉
}

fn main4_2() {
    // 此绑定生存于 main 函数中
    let long_lived_binding = 1;

    // 这是一个代码块，比 main 函数拥有更小的作用域
    {
        // 此绑定只存在于本代码块
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // 此绑定*遮蔽*了外面的绑定
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }
    // 代码块结束

    // 报错！`short_lived_binding` 在此作用域上不存在
    // println!("outer short: {}", short_lived_binding);
    // 改正 ^ 注释掉这行

    println!("outer long: {}", long_lived_binding);

    // 此绑定同样*遮蔽*了前面的绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}
fn main4_3() {
    // 声明一个变量绑定
    let a_binding;

    {
        let x = 2;

        // 初始化一个绑定
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // 报错！使用了未初始化的绑定
    // println!("another binding: {}", another_binding);
    // 改正 ^ 注释掉此行

    another_binding = 1;

    println!("another binding: {}", another_binding);
}

fn main4_4() {
    let mut _mutable_integer = 7i32;

    {
        // 被不可变的 `_mutable_integer` 遮蔽
        let _mutable_integer = _mutable_integer;

        // 报错！`_mutable_integer` 在本作用域被冻结
        // _mutable_integer = 50;
        // 改正 ^ 注释掉上面这行

        // `_mutable_integer` 离开作用域
    }

    // 正常运行！ `_mutable_integer` 在这个作用域没有冻结
    _mutable_integer = 3;
}


fn main5_1() {
    // 不显示类型转换产生的溢出警告。
    #![allow(overflowing_literals)]
    let decimal = 65.4321_f32;

    // 错误！不提供隐式转换
    // let integer: u8 = decimal;
    // 改正 ^ 注释掉这一行

    // 可以显式转换
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 当把任何类型转换为无符号类型 T 时，会不断加上或减去 (std::T::MAX + 1)
    // 直到值位于新类型 T 的范围内。

    // 1000 已经在 u16 的范围内
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // 事实上的处理方式是：从最低有效位（LSB，least significant bits）开始保留
    // 8 位，然后剩余位置，直到最高有效位（MSB，most significant bit）都被抛弃。
    // 译注：MSB 就是二进制的最高位，LSB 就是二进制的最低位，按日常书写习惯就是
    // 最左边一位和最右边一位。
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // 对正数，这就和取模一样。
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 当转换到有符号类型时，（位操作的）结果就和 “先转换到对应的无符号类型，
    // 如果 MSB 是 1，则该值为负” 是一样的。

    // 当然如果数值已经在目标类型的范围内，就直接把它放进去。
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 转成 u8 还是 128，但转到 i8 相当于给 128 取八位的二进制补码，其值是：
    println!(" 128 as a i8 is : {}", 128 as i8);

    // 重复之前的例子
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // 232 的二进制补码是 -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}

fn main5_2() {
    // 带后缀的字面量，其类型在初始化时已经知道了。
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面量，其类型取决于如何使用它们。
    let i = 1;
    let f = 1.0;

    // `size_of_val` 返回一个变量所占的字节数
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

fn main5_3() {
    // 因为有类型说明，编译器知道 `elem` 的类型是 u8。
    let elem = 5u8;

    // 创建一个空向量（vector，即不定长的，可以增长的数组）。
    let mut vec = Vec::new();
    // 现在编译器还不知道 `vec` 的具体类型，只知道它是某种东西构成的向量（`Vec<_>`）

    // 在向量中插入 `elem`。
    vec.push(elem);
    // 啊哈！现在编译器知道 `vec` 是 u8 的向量了（`Vec<u8>`）。
    // 试一试 ^ 注释掉 `vec.push(elem)` 这一行。

    println!("{:?}", vec);
}
fn main5_4() {
    // `NanoSecond` 是 `u64` 的新名字。
    type NanoSecond = u64;
    type Inch = u64;

    // 通过这个属性屏蔽警告。
    #[allow(non_camel_case_types)]
    type u64_t = u64;
    // 试一试 ^ 移除上面那个属性

    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // 注意类型别名*并不能*提供额外的类型安全，因为别名*并不是*新的类型。
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}

fn main6_1() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }
    let num = Number::from(30);
    println!("My number is {:?}", num);



    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    // let num = int.into();
    println!("My number is {:?}", num);
}

fn main6_2() {
    use std::convert::TryFrom;
    use std::convert::TryInto;

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
fn main6_3() {
    struct Circle {
        radius: i32
    }

    impl ToString for Circle {
        fn to_string(&self) -> String {
            format!("Circle of radius {:?}", self.radius)
        }
    }
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    // let parsed1 = "22".parse();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};
}

fn main7() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 将此表达式赋给 `y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号结束了这个表达式，于是将 `()` 赋给 `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

fn main8_1() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // 这个表达式返回一个 `i32` 类型。
            10 * n
        } else {
            println!(", and is a big number, half the number");

            // 这个表达式也必须返回一个 `i32` 类型。
            n / 2
            // 试一试 ^ 试着加上一个分号来结束这条表达式。
        };
    //   ^ 不要忘记在这里加上一个分号！所有的 `let` 绑定都需要它。

    println!("{} -> {}", n, big_n);
}

fn main8_2() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过这次迭代的剩下内容
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // 退出循环
            break;
        }
    }
}
fn main8_2_1() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // 这只是中断内部的循环
            //break;

            // 这会中断外层循环
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

fn main8_2_2() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

fn main8_3() {
    // 计数器变量
    let mut n = 1;

    // 当 `n` 小于 101 时循环
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器值加 1
        n += 1;
    }
}

fn main8_5() {
    let number = 13;
    // 试一试 ^ 将不同的值赋给 `number`

    println!("Tell me about {}", number);
    match number {
        // 匹配单个值
        1 => println!("One!"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 试一试 ^ 将 13 添加到质数列表中
        // 匹配一个闭区间范围
        13..=19 => println!("A teen"),
        // 处理其他情况
        _ => println!("Ain't special"),
        // 试一试 ^ 注释掉这个总括性的分支
    }

    let boolean = true;
    // match 也是一个表达式
    let binary = match boolean {
        // match 分支必须覆盖所有可能的值
        false => 0,
        true => 1,
        // 试一试 ^ 将其中一条分支注释掉
    };

    println!("{} -> {}", boolean, binary);
}

fn main8_5_1() {
    let triple = (0, -2, 3);
    // 试一试 ^ 将不同的值赋给 `triple`

    println!("Tell me about {:?}", triple);
    // match 可以解构一个元组
    match triple {
        // 解构出第二个和第三个元素
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        // `..` 可用来忽略元组的其余部分
        _      => println!("It doesn't matter what they are"),
        // `_` 表示不将值绑定到变量
    }
    // 需要 `allow` 来消除警告，因为只使用了枚举类型的一种取值。
    enum Color {
        // 这三个取值仅由它们的名字（而非类型）来指定。
        Red,
        Blue,
        Green,
        // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);
    // 试一试 ^ 将不同的值赋给 `color`

    println!("What color is it?");
    // 可以使用 `match` 来解构 `enum`。
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                     c, m, y, k),
        // 不需要其它分支，因为所有的情形都已覆盖
    }
}
fn main8_5_1_3() {
    // 获得一个 `i32` 类型的引用。`&` 表示取引用。
    let reference = &4;

    match reference {
        // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
        // `&i32`（译注：即 `reference` 的类型）
        // `&val`（译注：即用于匹配的模式）
        // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
        // 译注：因此可用 `val` 表示被 `reference` 引用的值 4。
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // 如果不想用 `&`，需要在匹配前解引用。
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // 如果一开始就不用引用，会怎样？ `reference` 是一个 `&` 类型，因为赋值语句
    // 的右边已经是一个引用。但下面这个不是引用，因为右边不是。
    let _not_a_reference = 3;
    let not_a_reference = 3;

    // Rust 对这种情况提供了 `ref`。它更改了赋值行为，从而可以对具体值创建引用。
    // 下面这行将得到一个引用。
    let ref _is_a_reference = 3;
    let ref is_a_reference = 3;

    // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
    let value = 5;
    let mut mut_value = 6;

    // 使用 `ref` 关键字来创建引用。
    // 译注：下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上
    // 似乎看不出什么区别。但读者可以把 `println!` 中的 `r` 改成 `*r`，仍然能
    // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解
    // 引用。
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // 类似地使用 `ref mut`。
    match mut_value {
        ref mut m => {
            // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
fn main8_5_1_4() {
    struct Foo { x: (u32, u32), y: u32 }

    // 解构结构体的成员
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {},  y = {} ", a, b, y);

    // 可以解构结构体并重命名变量，成员顺序并不重要

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // 也可以忽略某些变量
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // 这将得到一个错误：模式中没有提及 `x` 字段
    // let Foo { y } = foo;

    struct Point {
        x: i32,
        y: i32
    }
    struct Rectangle {
        a: Point,
        b: Point
    }

    let rect1 = Rectangle{
        a: Point{
            x: 1,
            y: 1
        },
        b: Point{
            x: 3,
            y: 3
        }
    };

    let Rectangle{a:Point{x: x1, y: y1}, b:Point{x: x2,y: y2}} = rect1;

    println!("{} {} {} {}", x1, y1, x2, y2);
}

fn main8_5_2() {
    let pair = (2, -2);
    // 试一试 ^ 将不同的值赋给 `pair`

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // ^ `if` 条件部分是一个卫语句
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

fn main8_5_3() {
    // `age` 函数，返回一个 `u32` 值。
    fn age() -> u32 {
        15
    }

    println!("Tell me what type of person you are");

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // 可以直接匹配（`match`） 1 ..= 12，但那样的话孩子会是几岁？
        // 相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // 不符合上面的范围。返回结果。
        n             => println!("I'm an old person of age {:?}", n),
    }

    fn some_number() -> Option<u32> {
        Some(42)
        // todo Some? what is it?
    }

    match some_number() {
        // 得到 `Some` 可变类型，如果它的值（绑定到 `n` 上）等于 42，则匹配。
        Some(n @ 42) => println!("The Answer: {}!", n),
        // 匹配任意其他数字。
        Some(n)      => println!("Not interesting... {}", n),
        // 匹配任意其他值（`None` 可变类型）。
        _            => (),
    }
}

fn main8_6() {
    // 将 `optional` 定为 `Option<i32>` 类型
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
            // ^ 行首需要 2 层缩进。这里从 optional 中解构出 `i`。
            // 译注：正确的缩进是好的，但并不是 “不缩进就不能运行” 这个意思。
        },
        _ => {},
        // ^ 必须有，因为 `match` 需要覆盖全部情况。不觉得这行很多余吗？
    };

    // 全部都是 `Option<i32>` 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构读作：若 `let` 将 `number` 解构成 `Some(i)`，则执行
    // 语句块（`{}`）
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // 如果要指明失败情形，就使用 else：
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // 解构失败。切换到失败情形。
        println!("Didn't match a number. Let's go with a letter!");
    };

    // 提供另一种失败情况下的条件。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
        // 解构失败。使用 `else if` 来判断是否满足上面提供的条件。
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 条件的值为 false。于是以下是默认的分支：
        println!("I don't like letters. Let's go with an emoticon :)!");
    };

    // 以这个 enum 类型为例
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }

    // 创建变量
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // 变量 a 匹配到了 Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // 变量 b 没有匹配到 Foo::Bar，因此什么也不会打印。
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // 变量 c 匹配到了 Foo::Qux，它带有一个值，就和上面例子中的 Some() 类似。
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    let a = Foo::Bar;

    // 变量匹配 Foo::Bar
    /*
    if Foo::Bar == a {
        // ^-- 这就是编译时发现的错误。使用 `if let` 来替换它。
        println!("a is foobar");
    }

     */

}

fn main8_7() {
    // 将 `optional` 设为 `Option<i32>` 类型
    let mut optional = Some(0);

// 重复运行这个测试。
    loop {
        match optional {
            // 如果 `optional` 解构成功，就执行下面语句块。
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ 需要三层缩进！
            },
            // 当解构失败时退出循环：
            _ => { break; }
            // ^ 为什么必须写这样的语句呢？肯定有更优雅的处理方式！
        }
    }

    // 将 `optional` 设为 `Option<i32>` 类型
    let mut optional = Some(0);

    // 这读作：当 `let` 将 `optional` 解构成 `Some(i)` 时，就
    // 执行语句块（`{}`）。否则就 `break`。
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ 使用的缩进更少，并且不用显式地处理失败情况。
    }
    // ^ `if let` 有可选的 `else`/`else if` 分句，
    // 而 `while let` 没有。
}

fn main9() {
    // 一个返回布尔值的函数
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        // 边界情况，提前返回
        if rhs == 0 {
            return false;
        }

        // 这是一个表达式，可以不用 `return` 关键字
        lhs % rhs == 0
    }

    // 一个 “不” 返回值的函数。实际上会返回一个单元类型 `()`。
    fn fizzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // 当函数返回 `()` 时，函数签名可以省略返回类型
    fn fizzbuzz_to(n: u32) {
        for n in 1..=n {
            fizzbuzz(n);
        }
    }
    fizzbuzz_to(100);
}

fn main9_1() {
    struct Point {
        x: f64,
        y: f64,
    }

    // 实现的代码块，`Point` 的所有方法都在这里给出
    impl Point {
        // 这是一个静态方法（static method）
        // 静态方法不需要被实例调用
        // 这类方法一般用作构造器（constructor）
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        // 另外一个静态方法，需要两个参数：
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        // 这是一个实例方法（instance method）
        // `&self` 是 `self: &Self` 的语法糖（sugar），其中 `Self` 是方法调用者的
        // 类型。在这个例子中 `Self` = `Rectangle`
        fn area(&self) -> f64 {
            // `self` 通过点运算符来访问结构体字段
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            // `abs` 是一个 `f64` 类型的方法，返回调用者的绝对值
            ((x1 - x2) * (y1 - y2)).abs()

            // ((self.p1.x - self.p2.x)*(self.p1.y - self.p2.y)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        // 这个方法要求调用者是可变的
        // `&mut self` 为 `self: &mut Self` 的语法糖
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    // `Pair` 拥有资源：两个堆分配的整型
    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        // 这个方法会 “消耗” 调用者的资源
        // `self` 为 `self: Self` 的语法糖
        fn destroy(self) {
            // 解构 `self`
            let Pair(first, second) = self;

            println!("Destroying Pair({}, {})", first, second);

            // `first` 和 `second` 离开作用域后释放
        }
    }

    let rectangle = Rectangle {
        // 静态方法使用双冒号调用
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 实例方法通过点运算符来调用
    // 注意第一个参数 `&self` 是隐式传递的，亦即：
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // 报错！ `rectangle` 是不可变的，但这方法需要一个可变对象
    //rectangle.translate(1.0, 0.0);
    // 试一试 ^ 去掉此行的注释

    // 正常运行！可变对象可以调用可变方法
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // 报错！前面的 `destroy` 调用 “消耗了” `pair`
    // pair.destroy();
    // 试一试 ^ 将此行注释去掉
}

fn main9_2() {
    // 通过闭包和函数分别实现自增。
    // 译注：下面这行是使用函数的实现
    fn  function            (i: i32) -> i32 { i + 1 }

    // 闭包是匿名的，这里我们将它们绑定到引用。
    // 类型标注和函数的一样，不过类型标注和使用 `{}` 来围住函数体都是可选的。
    // 这些匿名函数（nameless function）被赋值给合适地命名的变量。
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    // 译注：将闭包绑定到引用的说法可能不准。
    // 据[语言参考](https://doc.rust-lang.org/beta/reference/types.html#closure-types)
    // 闭包表达式产生的类型就是 “闭包类型”，不属于引用类型，而且确实无法对上面两个
    // `closure_xxx` 变量解引用。

    let i = 1;
    // 调用函数和闭包。
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // 没有参数的闭包，返回一个 `i32` 类型。
    // 返回类型是自动推导的。
    let one = || 1;
    println!("closure returning one: {}", one());

    let my = || {println!("hello"); 1};

    println!("{}", my());
}

fn main9_2_1() {
    use std::mem;

    let color = String::from("green");

    // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color` 会一直保持被借用状态直到
    // `print` 离开作用域。
    //
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let print = || println!("`color`: {}", color);

    // 使用借用来调用闭包 `color`。
    print();

    // `color` 可再次被不可变借用，因为闭包只持有一个指向 `color` 的不可变引用。
    let _reborrow = &color;
    print();

    // 在最后使用 `print` 之后，移动或重新借用都是允许的。
    let _color_moved = color;

    let mut count = 0;
    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 使用可变借用调用闭包
    inc();

    // 因为之后调用闭包，所以仍然可变借用 `count`
    // 试图重新借用将导致错误
    // let _reborrow = &count;
    // ^ 试一试：将此行注释去掉。
    inc();

    // 闭包不再借用 `&mut count`，因此可以正确地重新借用
    let _count_reborrowed = &mut count;

    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
    //consume();
    // ^ 试一试：将此行注释去掉。

    // `Vec` 在语义上是不可复制的。
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    //println!("There're {} elements in vec", haystack.len());
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走
    // 之后继续使用它。

    // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `haystack`，因此之后
    // `haystack` 仍然可用，取消上面的注释也不会导致错误。
}

fn main9_2_2() {
    // 该函数将闭包作为参数并调用它。
    fn apply<F>(f: F) where
    // 闭包没有输入值和返回值。
        F: FnOnce() {
        // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。

        f();
    }

    // 输入闭包，返回一个 `i32` 整型的函数。
    fn apply_to_3<F>(f: F) -> i32 where
    // 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
        F: Fn(i32) -> i32 {

        f(3)
    }

    use std::mem;

    let greeting = "hello";
    // 不可复制的类型。
    // `to_owned` 从借用的数据创建有所有权的数据。
    let mut farewell = "goodbye".to_owned();

    // 捕获 2 个变量：通过引用捕获 `greeting`，通过值捕获 `farewell`。
    let diary = || {
        // `greeting` 通过引用捕获，故需要闭包是 `Fn`。
        println!("I said {}.", greeting);

        // 下文改变了 `farewell` ，因而要求闭包通过可变引用来捕获它。
        // 现在需要 `FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用 drop 又要求闭包通过值获取 `farewell`。
        // 现在需要 `FnOnce`。
        mem::drop(farewell);
    };

    // 以闭包作为参数，调用函数 `apply`。
    apply(diary);

    // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
// todo: now pass 9.2

fn main9_3() {
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // 命令式（imperative）的写法
    // 声明累加器变量
    let mut acc = 0;
    // 迭代：0，1, 2, ... 到无穷大
    for n in 0.. {
        // 数字的平方
        let n_squared = n * n;

        if n_squared >= upper {
            // 若大于上限则退出循环
            break;
        }

        if is_odd(n_squared) {
            // 如果是奇数就计数
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // 函数式的写法
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)             // 所有自然数取平方
            .take_while(|&n| n < upper) // 取小于上限的
            .filter(|&n| is_odd(n))     // 取奇数
            .fold(0, |sum, i| sum + i); // 最后加起来
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

fn main9_4() {
    fn foo() -> ! {
        panic!("This call never returns.");
    }

    fn some_fn() {
        ()
    }
    let a: () = some_fn();
    println!("This function returns and you can see this line.");

    // let x: ! = panic!("This call never returns.");
    println!("You will never see this line!");
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // 注意这个 match 表达式的返回值必须为 u32，
            // 因为 “addition” 变量是这个类型。
            let addition: u32 = match i%2 == 1 {
                // “i” 变量的类型为 u32，这毫无问题。
                true => i,
                // 另一方面，“continue” 表达式不返回 u32，但它仍然没有问题，
                // 因为它永远不会返回，因此不会违反匹配表达式的类型要求。
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
// 一个名为 `my_mod` 的模块
mod my_mod {
    use std::{print, println};

    // 模块中的项默认具有私有的可见性
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // 使用 `pub` 修饰语来改变默认可见性。
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 在同一模块中，项可以访问其它项，即使它是私有的。
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
            public_function_in_nested()
        }

        // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested");
        }

        // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // `pub(crate)` 使得函数只在当前 crate 中可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    // 嵌套模块的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}
fn main10() {
    // 模块机制消除了相同名字的项之间的歧义。
    function();
    my_mod::function();

    // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项可以在同一个 crate 中的任何地方访问
    my_mod::public_function_in_crate();

    // pub(in path) 项只能在指定的模块中访问
    // 报错！函数 `public_function_in_my_mod` 是私有的
    // my_mod::nested::public_function_in_my_mod();
    // 试一试 ^ 取消该行的注释

    // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的

    // 报错！`private_function` 是私有的
    //my_mod::private_function();
    // 试一试 ^ 取消此行注释

    // 报错！`private_function` 是私有的
    //my_mod::nested::private_function();
    // 试一试 ^ 取消此行的注释

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
    // 试一试 ^ 取消此行的注释
}

mod my10_2 {
    // 一个公有的结构体，带有一个公有的字段（类型为泛型 `T`）
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // 一个公有的结构体，带有一个私有的字段（类型为泛型 `T`）
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // 一个公有的构造器方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn main10_2() {
    // 带有公有字段的公有结构体，可以像平常一样构造
    let open_box = my10_2::OpenBox { contents: "public information" };

    // 并且它们的字段可以正常访问到。
    println!("The open box contains: {}", open_box.contents);

    // 带有私有字段的公有结构体不能使用字段名来构造。
    // 报错！`ClosedBox` 含有私有字段。
    //let closed_box = my::ClosedBox { contents: "classified information" };
    // 试一试 ^ 取消此行注释


    // 不过带有私有字段的结构体可以使用公有的构造器来创建。
    let _closed_box = my10_2::ClosedBox::new("classified information");

    // 并且一个结构体中的私有字段不能访问到。
    // 报错！`content` 字段是私有的。
    // println!("The closed box contains: {}", _closed_box.contents);
    // 试一试 ^ 取消此行注释

}
// 将 `deeply::nested::function` 路径绑定到 `other_function`。
use deeply::nested::function as other_function;

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`")
        }
    }
}

fn main10_3() {
    // 更容易访问 `deeply::nested::funcion`
    other_function();

    println!("Entering block");
    {
        // 这和 `use deeply::nested::function as function` 等价。
        // 此 `function()` 将遮蔽外部的同名函数。
        use deeply::nested::function;
        function();

        // `use` 绑定拥有局部作用域。在这个例子中，`function()`
        // 的遮蔽只存在在这个代码块中。
        println!("Leaving block");
    }

    function();
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // 让我们从这个作用域中访问所有名为 `function` 的函数！
        print!("called `my::indirect_call()`, that\n> ");

        // `self` 关键字表示当前的模块作用域——在这个例子是 `my`。
        // 调用 `self::function()` 和直接调用 `function()` 都得到相同的结果，
        // 因为他们表示相同的函数。
        self::function();
        function();

        // 我们也可以使用 `self` 来访问 `my` 内部的另一个模块：
        self::cool::function();

        // `super` 关键字表示父作用域（在 `my` 模块外面）。
        super::function();

        // 这将在 *crate* 作用域内绑定 `cool::function` 。
        // 在这个例子中，crate 作用域是最外面的作用域。
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main10_4() {
    my::indirect_call();
}

mod my10_5;

fn main10_5() {
    my10_5::function();
}

fn main11_2() {
    // rary::public_function();

    // 报错！ `private_function` 是私有的
    //rary::private_function();

    // rary::indirect_access();
}

pub mod chapter13 {
    // 这个函数仅当目标系统是 Linux 的时候才会编译
    #[cfg(target_os = "linux")]
    fn are_you_on_linux() {
        println!("You are running linux!")
    }

    // 而这个函数仅当目标系统 **不是** Linux 时才会编译
    #[cfg(not(target_os = "linux"))]
    fn are_you_on_linux() {
        println!("You are *not* running linux!")
    }

    pub fn main13_3() {
        are_you_on_linux();

        println!("Are you sure?");
        if cfg!(target_os = "linux") {
            println!("Yes. It's definitely linux!");
        } else {
            println!("Yes. It's definitely *not* linux!");
        }
    }
}

fn main13_1() {
    fn used_function() {}

    // `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
    #[allow(dead_code)]
    fn unused_function() {}

    #[allow(dead_code)]
    fn noisy_unused_function() {}
    // 改正 ^ 增加一个属性来消除警告

    used_function();
}
mod c14 {
    // 一个具体类型 `A`。
    struct A;

    // 在定义类型 `Single` 时，第一次使用类型 `A` 之前没有写 `<A>`。
// 因此，`Single` 是个具体类型，`A` 取上面的定义。
    struct Single(A);
//            ^ 这里是 `Single` 对类型 `A` 的第一次使用。

    // 此处 `<T>` 在第一次使用 `T` 前出现，所以 `SingleGen` 是一个泛型类型。
// 因为 `T` 是泛型的，所以它可以是任何类型，包括在上面定义的具体类型 `A`。
    struct SingleGen<T>(T);
    pub fn main() {
        // `Single` 是具体类型，并且显式地使用类型 `A`。
        let _s = Single(A);

        // 创建一个 `SingleGen<char>` 类型的变量 `_char`，并令其值为 `SingleGen('a')`
        // 这里的 `SingleGen` 的类型参数是显式指定的。
        let _char: SingleGen<char> = SingleGen('a');

        // `SingleGen` 的类型参数也可以隐式地指定。
        let _t    = SingleGen(A); // 使用在上面定义的 `A`。
        let _i32  = SingleGen(6); // 使用 `i32` 类型。
        let _char = SingleGen('a'); // 使用 `char`。
    }
    pub mod c14_1 {
        struct A;          // 具体类型 `A`。
        struct S(A);       // 具体类型 `S`。
        struct SGen<T>(T); // 泛型类型 `SGen`。

        // 下面全部函数都得到了变量的所有权，并立即使之离开作用域，将变量释放。

        // 定义一个函数 `reg_fn`，接受一个 `S` 类型的参数 `_s`。
        // 因为没有 `<T>` 这样的泛型类型参数，所以这不是泛型函数。
        fn reg_fn(_s: S) {}

        // 定义一个函数 `gen_spec_t`，接受一个 `SGen<A>` 类型的参数 `_s`。
        // `SGen<>` 显式地接受了类型参数 `A`，且在 `gen_spec_t` 中，`A` 没有被用作
        // 泛型类型参数，所以函数不是泛型的。
        fn gen_spec_t(_s: SGen<A>) {}

        // 定义一个函数 `gen_spec_i32`，接受一个 `SGen<i32>` 类型的参数 `_s`。
        // `SGen<>` 显式地接受了类型参量 `i32`，而 `i32` 是一个具体类型。
        // 由于 `i32` 不是一个泛型类型，所以这个函数也不是泛型的。
        fn gen_spec_i32(_s: SGen<i32>) {}

        // 定义一个函数 `generic`，接受一个 `SGen<T>` 类型的参数 `_s`。
        // 因为 `SGen<T>` 之前有 `<T>`，所以这个函数是关于 `T` 的泛型函数。
        fn generic<T>(_s: SGen<T>) {}

        fn main() {
            // 使用非泛型函数
            reg_fn(S(A));          // 具体类型。
            gen_spec_t(SGen(A));   // 隐式地指定类型参数 `A`。
            gen_spec_i32(SGen(6)); // 隐式地指定类型参数 `i32`。

            // 为 `generic()` 显式地指定类型参数 `char`。
            generic::<char>(SGen('a'));

            // 为 `generic()` 隐式地指定类型参数 `char`。
            generic(SGen('c'));
        }

    }
    pub mod c14_2 {
        struct S; // 具体类型 `S`
        struct GenericVal<T>(T,); // 泛型类型 `GenericVal`

        // GenericVal 的 `impl`，此处我们显式地指定了类型参数：
        impl GenericVal<f32> {} // 指定 `f32` 类型
        impl GenericVal<S> {} // 指定为上面定义的 `S`

        // `<T>` 必须在类型之前写出来，以使类型 `T` 代表泛型。
        impl <T> GenericVal<T> {}

        struct Val {
            val: f64
        }

        struct GenVal<T>{
            gen_val: T
        }

        // Val 的 `impl`
        impl Val {
            fn value(&self) -> &f64 { &self.val }
        }

        // GenVal 的 `impl`，指定 `T` 是泛型类型
        impl <T> GenVal<T> {
            fn value(&self) -> &T { &self.gen_val }
        }

        fn main() {
            let x = Val { val: 3.0 };
            let y = GenVal { gen_val: 3i32 };

            println!("{}, {}", x.value(), y.value());
        }

    }

    pub mod c14_3 {
        // 不可复制的类型。
        struct Empty;
        struct Null;

        // `T` 的泛型 trait。
        trait DoubleDrop<T> {
            // 定义一个调用者的方法，接受一个额外的参数 `T`，但不对它做任何事。
            fn double_drop(self, _: T);
        }

        // 对泛型的调用者类型 `U` 和任何泛型类型 `T` 实现 `DoubleDrop<T>` 。
        impl<T, U> DoubleDrop<T> for U {
            // 此方法获得两个传入参数的所有权，并释放它们。
            fn double_drop(self, _: T) {}
        }

        fn main() {
            let empty = Empty;
            let null  = Null;

            // 释放 `empty` 和 `null`。
            empty.double_drop(null);

            // empty;
            // null;
            // ^ 试一试：去掉这两行的注释。
        }
    }

    pub mod c14_4 {
        use std::fmt::Display;

        // 定义一个函数 `printer`，接受一个类型为泛型 `T` 的参数，
        // 其中 `T` 必须实现 `Display` trait。
        fn printer<T: Display>(t: T) {
            println!("{}", t);
        }

        struct S<T: Display>(T);

        // 报错！`Vec<T>` 未实现 `Display`。此次泛型具体化失败。
        // let s = S(vec![1]);
        // 这个 trait 用来实现打印标记：`{:?}`。
        use std::fmt::Debug;

        trait HasArea {
            fn area(&self) -> f64;
        }

        impl HasArea for Rectangle {
            fn area(&self) -> f64 { self.length * self.height }
        }

        #[derive(Debug)]
        struct Rectangle { length: f64, height: f64 }
        #[allow(dead_code)]
        struct Triangle  { length: f64, height: f64 }

        // 泛型 `T` 必须实现 `Debug` 。只要满足这点，无论什么类型
        // 都可以让下面函数正常工作。
        fn print_debug<T: Debug>(t: &T) {
            println!("{:?}", t);
        }

        // `T` 必须实现 `HasArea`。任意符合该约束的泛型的实例
        // 都可访问 `HasArea` 的 `area` 函数
        fn area<T: HasArea>(t: &T) -> f64 { t.area() }

        pub(crate) fn main() {
            let rectangle = Rectangle { length: 3.0, height: 4.0 };
            let _triangle = Triangle  { length: 3.0, height: 4.0 };

            print_debug(&rectangle);
            println!("Area: {}", area(&rectangle));

            //print_debug(&_triangle);
            //println!("Area: {}", area(&_triangle));
            // ^ 试一试：取消上述语句的注释。
            // | 报错：未实现 `Debug` 或 `HasArea`。
        }

    }

    pub mod c14_5 {
        use std::fmt::{Debug, Display};

        fn compare_prints<T: Debug + Display>(t: &T) {
            println!("Debug: `{:?}`", t);
            println!("Display: `{}`", t);
        }

        fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
            println!("t: `{:?}", t);
            println!("u: `{:?}", u);
        }

        pub(crate) fn main() {
            let string = "words";
            let array = [1, 2, 3];
            let vec = vec![1, 2, 3];

            compare_prints(&string);
            //compare_prints(&array);
            // 试一试 ^ 将此行注释去掉。

            compare_types(&array, &vec);
        }

    }

    pub mod c14_6 {
        use std::fmt::Debug;

        trait PrintInOption {
            fn print_in_option(self);
        }

        // 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），
        // 或着改用另一种间接的方法。
        impl<T> PrintInOption for T where
            Option<T>: Debug {
            // 我们要将 `Option<T>: Debug` 作为约束，因为那是要打印的内容。
            // 否则我们会给出错误的约束。
            fn print_in_option(self) {
                println!("{:?}", Some(self));
            }
        }

        pub(crate) fn main() {
            let vec = vec![1, 2, 3];

            vec.print_in_option();
        }
    }

    pub mod c14_7 {
        struct Years(i64);

        struct Days(i64);

        impl Years {
            pub fn to_days(&self) -> Days {
                Days(self.0 * 365)
            }
        }


        impl Days {
            /// 舍去不满一年的部分
            pub fn to_years(&self) -> Years {
                Years(self.0 / 365)
            }
        }

        fn old_enough(age: &Years) -> bool {
            age.0 >= 18
        }

        pub(crate) fn main() {
            let age = Years(5);
            let age_days = age.to_days();
            println!("Old enough {}", old_enough(&age));
            println!("Old enough {}", old_enough(&age_days.to_years()));
            // println!("Old enough {}", old_enough(&age_days));
        }
    }

    pub mod c14_8_1 {
        struct Container(i32, i32);

        // 这个 trait 检查给定的 2 个项是否储存于容器中
        // 并且能够获得容器的第一个或最后一个值。
        trait Contains<A, B> {
            fn contains(&self, _: &A, _: &B) -> bool; // 显式地要求 `A` 和 `B`
            fn first(&self) -> i32; // 未显式地要求 `A` 或 `B`
            fn last(&self) -> i32;  // 未显式地要求 `A` 或 `B`
        }

        impl Contains<i32, i32> for Container {
            // 如果存储的数字和给定的相等则为真。
            fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
                (&self.0 == number_1) && (&self.1 == number_2)
            }

            // 得到第一个数字。
            fn first(&self) -> i32 { self.0 }

            // 得到最后一个数字。
            fn last(&self) -> i32 { self.1 }
        }

        // 容器 `C` 就包含了 `A` 和 `B` 类型。鉴于此，必须指出 `A` 和 `B` 显得很麻烦。
        fn difference<A, B, C>(container: &C) -> i32 where
            C: Contains<A, B> {
            container.last() - container.first()
        }

        pub(crate) fn main() {
            let number_1 = 3;
            let number_2 = 10;

            let container = Container(number_1, number_2);

            println!("Does container contain {} and {}: {}",
                     &number_1, &number_2,
                     container.contains(&number_1, &number_2));
            println!("First number: {}", container.first());
            println!("Last number: {}", container.last());

            println!("The difference is: {}", difference(&container));
        }

    }
    pub(crate) mod c14_8_2 {
        // `A` 和 `B` 在 trait 里面通过 `type` 关键字来定义。
        // （注意：此处的 `type` 不同于为类型取别名时的 `type`）。
        // trait Contains {
        //     type A;
        //     type B;
        //
        //     // 这种语法能够泛型地表示这些新类型。
        //     fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        // }

        struct Container(i32, i32);

        // 这个 trait 检查给定的 2 个项是否储存于容器中
        // 并且能够获得容器的第一个或最后一个值。
        trait Contains {
            // 在这里定义可以被方法使用的泛型类型。
            type A;
            type B;

            fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
            fn first(&self) -> i32;
            fn last(&self) -> i32;
        }

        impl Contains for Container {
            // 指出 `A` 和 `B` 是什么类型。如果 `input`（输入）类型
            // 为 `Container(i32, i32)`，那么 `output`（输出）类型
            // 会被确定为 `i32` 和 `i32`。
            type A = i32;
            type B = i32;

            // `&Self::A` 和 `&Self::B` 在这里也是合法的类型。
            fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
                (&self.0 == number_1) && (&self.1 == number_2)
            }

            // 得到第一个数字。
            fn first(&self) -> i32 { self.0 }

            // 得到最后一个数字。
            fn last(&self) -> i32 { self.1 }
        }

        fn difference<C: Contains>(container: &C) -> i32 {
            container.last() - container.first()
        }

        pub(crate) fn main() {
            let number_1 = 3;
            let number_2 = 10;

            let container = Container(number_1, number_2);

            println!("Does container contain {} and {}: {}",
                     &number_1, &number_2,
                     container.contains(&number_1, &number_2));
            println!("First number: {}", container.first());
            println!("Last number: {}", container.last());

            println!("The difference is: {}", difference(&container));
        }
    }
    pub mod c14_9 {
        use std::marker::PhantomData;

        // 这个虚元组结构体对 `A` 是泛型的，并且带有隐藏参数 `B`。
        #[derive(PartialEq)] // 允许这种类型进行相等测试（equality test）。
        struct PhantomTuple<A, B>(A,PhantomData<B>);

        // 这个虚类型结构体对 `A` 是泛型的，并且带有隐藏参数 `B`。
        #[derive(PartialEq)] // 允许这种类型进行相等测试。
        struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

        // 注意：对于泛型 `A` 会分配存储空间，但 `B` 不会。
        //       因此，`B` 不能参与运算。

        pub(crate) fn main() {
            // 这里的 `f32` 和 `f64` 是隐藏参数。
            // 被指定为 `<char, f32>` 的 `PhantomTuple` 类型。
            let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
            // 被指定为 `<char, f64>` `PhantomTuple` 类型。
            let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

            // 被指定为 `<char, f32>` 的类型。
            let _struct1: PhantomStruct<char, f32> = PhantomStruct {
                first: 'Q',
                phantom: PhantomData,
            };
            // 被指定为 `<char, f64>` 的类型。
            let _struct2: PhantomStruct<char, f64> = PhantomStruct {
                first: 'Q',
                phantom: PhantomData,
            };

            // 编译期错误！类型不匹配，所以这些值不能够比较：
            //println!("_tuple1 == _tuple2 yields: {}",
            //          _tuple1 == _tuple2);

            // 编译期错误！类型不匹配，所以这些值不能够比较：
            //println!("_struct1 == _struct2 yields: {}",
            //          _struct1 == _struct2);
        }

    }

    pub mod c14_9_1 {
        use std::ops::Add;
        use std::marker::PhantomData;

        /// 创建空枚举类型来表示单位。
        #[derive(Debug, Clone, Copy)]
        enum Inch {}
        #[derive(Debug, Clone, Copy)]
        enum Mm {}

        /// `Length` 是一个带有虚类型参数 `Unit` 的类型，
        /// 而且对于表示长度的类型（即 `f64`）而言，`Length` 不是泛型的。
        ///
        /// `f64` 已经实现了 `Clone` 和 `Copy` trait.
        #[derive(Debug, Clone, Copy)]
        struct Length<Unit>(f64, PhantomData<Unit>);

        /// `Add` trait 定义了 `+` 运算符的行为。
        impl<Unit> Add for Length<Unit> {
            type Output = Length<Unit>;

            // add() 返回一个含有和的新的 `Length` 结构体。
            fn add(self, rhs: Length<Unit>) -> Length<Unit> {
                // `+` 调用了针对 `f64` 类型的 `Add` 实现。
                Length(self.0 + rhs.0, PhantomData)
            }
        }

        pub fn main() {
            // 指定 `one_foot` 拥有虚类型参数 `Inch`。
            let one_foot:  Length<Inch> = Length(12.0, PhantomData);
            // `one_meter` 拥有虚类型参数 `Mm`。
            let one_meter: Length<Mm>   = Length(1000.0, PhantomData);

            // `+` 调用了我们对 `Length<Unit>` 实现的 `add()` 方法。
            //
            // 由于 `Length` 了实现了 `Copy`，`add()` 不会消耗 `one_foot`
            // 和 `one_meter`，而是复制它们作为 `self` 和 `rhs`。
            let two_feet = one_foot + one_foot;
            let two_meters = one_meter + one_meter;

            // 加法正常执行。
            println!("one foot + one_foot = {:?} in", two_feet.0);
            println!("one meter + one_meter = {:?} mm", two_meters.0);

            // 无意义的运算当然会失败：
            // 编译期错误：类型不匹配。
            //let one_feter = one_foot + one_meter;
        }
    }
}

pub mod c15 {
    pub mod c15_1 {
        // raii.rs
        fn create_box() {
            // 在堆上分配一个整型数据
            let _box1 = Box::new(3i32);

            // `_box1` 在这里被销毁，内存得到释放
        }

        pub fn main() {
            // 在堆上分配一个整型数据
            let _box2 = Box::new(5i32);

            // 嵌套作用域：
            {
                // 在堆上分配一个整型数据
                let _box3 = Box::new(4i32);

                // `_box3` 在这里被销毁，内存得到释放
            }

            // 创建一大堆 box（只是因为好玩）。
            // 完全不需要手动释放内存！
            for _ in 0u32..1_000 {
                create_box();
            }

            // `_box2` 在这里被销毁，内存得到释放
        }

        struct ToDrop;

        impl Drop for ToDrop {
            fn drop(&mut self) {
                println!("ToDrop is being dropped");
            }
        }

        pub fn main2() {
            let x = ToDrop;
            println!("Made a ToDrop!");
        }
    }

    pub mod c15_2 {
        // 此函数取得堆分配的内存的所有权
        fn destroy_box(c: Box<i32>) {
            println!("Destroying a box that contains {}", c);

            // `c` 被销毁且内存得到释放
        }

        pub(crate) fn main() {
            // 栈分配的整型
            let x = 5u32;

            // 将 `x` *复制*到 `y`——不存在资源移动
            let y = x;

            // 两个值各自都可以使用
            println!("x is {}, and y is {}", x, y);

            // `a` 是一个指向堆分配的整数的指针
            let a = Box::new(5i32);

            println!("a contains: {}", a);

            // *移动* `a` 到 `b`
            let b = a;
            // 把 `a` 的指针地址（而非数据）复制到 `b`。现在两者都指向
            // 同一个堆分配的数据，但是现在是 `b` 拥有它。

            // 报错！`a` 不能访问数据，因为它不再拥有那部分堆上的内存。
            // println!("a contains: {}", a);
            // 试一试 ^ 去掉此行注释

            // 此函数从 `b` 中取得堆分配的内存的所有权
            destroy_box(b);

            // 此时堆内存已经被释放，这个操作会导致解引用已释放的内存，而这是编译器禁止的。
            // 报错！和前面出错的原因一样。
            // println!("b contains: {}", b);
            // 试一试 ^ 去掉此行注释
        }
    }
    pub mod c15_2_1 {
        pub fn main() {
            let immutable_box = Box::new(5u32);

            println!("immutable_box contains {}", immutable_box);

            // 可变性错误
            //*immutable_box = 4;

            // *移动* box，改变所有权（和可变性）
            let mut mutable_box = immutable_box;

            println!("mutable_box contains {}", mutable_box);

            // 修改 box 的内容
            *mutable_box = 4;

            println!("mutable_box now contains {}", mutable_box);
        }
    }
    pub mod c15_2_2 {
        pub fn main() {
            #[derive(Debug)]
            struct Person {
                name: String,
                age: u8,
            }

            let person = Person {
                name: String::from("Alice"),
                age: 20,
            };

            // `name` 从 person 中移走，但 `age` 只是引用
            let Person { name, ref age } = person;

            println!("The person's age is {}", age);

            println!("The person's name is {}", name);

            // 报错！部分移动值的借用：`person` 部分借用产生
            // println!("The person struct is {:?}", person);

            // `person` 不能使用，但 `person.age` 因为没有被移动而可以继续使用
            println!("The person's age from person struct is {}", person.age);
        }
    }

    pub mod c15_3 {
        // 此函数取得一个 box 的所有权并销毁它
        fn eat_box_i32(boxed_i32: Box<i32>) {
            println!("Destroying box that contains {}", boxed_i32);
        }

        // 此函数借用了一个 i32 类型
        fn borrow_i32(borrowed_i32: &i32) {
            println!("This int is: {}", borrowed_i32);
        }

        pub fn main() {
            // 创建一个装箱的 i32 类型，以及一个存在栈中的 i32 类型。
            let boxed_i32 = Box::new(5_i32);
            let stacked_i32 = 6_i32;

            // 借用了 box 的内容，但没有取得所有权，所以 box 的内容之后可以再次借用。
            // 译注：请注意函数自身就是一个作用域，因此下面两个函数运行完成以后，
            // 在函数中临时创建的引用也就不复存在了。
            borrow_i32(&boxed_i32);
            borrow_i32(&stacked_i32);

            {
                // 取得一个对 box 中数据的引用
                let _ref_to_i32: &i32 = &boxed_i32;

                // 报错！
                // 当 `boxed_i32` 里面的值之后在作用域中被借用时，不能将其销毁。
                // eat_box_i32(boxed_i32);
                // 改正 ^ 注释掉此行

                // 在 `_ref_to_i32` 里面的值被销毁后，尝试借用 `_ref_to_i32`
                //（译注：如果此处不借用，则在上一行的代码中，eat_box_i32(boxed_i32)可以将 `boxed_i32` 销毁。）
                borrow_i32(_ref_to_i32);
                // `_ref_to_i32` 离开作用域且不再被借用。
            }

            // `boxed_i32` 现在可以将所有权交给 `eat_i32` 并被销毁。
            //（译注：能够销毁是因为已经不存在对 `boxed_i32` 的引用）
            eat_box_i32(boxed_i32);
        }

    }
    pub mod c15_3_1 {
        #[allow(dead_code)]
        #[derive(Clone, Copy)]
        struct Book {
            // `&'static str` 是一个对分配在只读内存区的字符串的引用
            author: &'static str,
            title: &'static str,
            year: u32,
        }

        // 此函数接受一个对 Book 类型的引用
        fn borrow_book(book: &Book) {
            println!("I immutably borrowed {} - {} edition", book.title, book.year);
        }

        // 此函数接受一个对可变的 Book 类型的引用，它把年份 `year` 改为 2014 年
        fn new_edition(book: &mut Book) {
            book.year = 2014;
            println!("I mutably borrowed {} - {} edition", book.title, book.year);
        }

        pub(crate) fn main() {
            // 创建一个名为 `immutabook` 的不可变的 Book 实例
            let immutabook = Book {
                // 字符串字面量拥有 `&'static str` 类型
                author: "Douglas Hofstadter",
                title: "Gödel, Escher, Bach",
                year: 1979,
            };

            // 创建一个 `immutabook` 的可变拷贝，命名为 `mutabook`
            let mut mutabook = immutabook;

            // 不可变地借用一个不可变对象
            borrow_book(&immutabook);

            // 不可变地借用一个可变对象
            borrow_book(&mutabook);

            // 可变地借用一个可变对象
            new_edition(&mut mutabook);

            // 报错！不能可变地借用一个不可变对象
            // new_edition(&mut immutabook);
            // 改正 ^ 注释掉此行
        }

    }
    pub mod c15_3_2{
        struct Point { x: i32, y: i32, z: i32 }

        pub(crate) fn main() {
            let mut point = Point { x: 0, y: 0, z: 0 };

            let borrowed_point = &point;
            let another_borrow = &point;

            // 数据可以通过引用或原始类型来访问
            println!("Point has coordinates: ({}, {}, {})",
                     borrowed_point.x, another_borrow.y, point.z);

            // 报错！`point` 不能以可变方式借用，因为当前还有不可变借用。
            // let mutable_borrow = &mut point;
            // TODO ^ 试一试去掉此行注释

            // 被借用的值在这里被重新使用
            println!("Point has coordinates: ({}, {}, {})",
                     borrowed_point.x, another_borrow.y, point.z);

            // 不可变的引用不再用于其余的代码，因此可以使用可变的引用重新借用。
            let mutable_borrow = &mut point;

            // 通过可变引用来修改数据
            mutable_borrow.x = 5;
            mutable_borrow.y = 2;
            mutable_borrow.z = 1;

            // 报错！不能再以不可变方式来借用 `point`，因为它当前已经被可变借用。
            // let y = &point.y;
            // TODO ^ 试一试去掉此行注释

            // 报错！无法打印，因为 `println!` 用到了一个不可变引用。
            // println!("Point Z coordinate is {}", point.z);
            // TODO ^ 试一试去掉此行注释

            // 正常运行！可变引用能够以不可变类型传入 `println!`
            println!("Point has coordinates: ({}, {}, {})",
                     mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

            // 可变引用不再用于其余的代码，因此可以重新借用
            let new_borrowed_point = &point;
            println!("Point now has coordinates: ({}, {}, {})",
                     new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
        }

    }

    pub mod c15_3_3 {
        #[derive(Clone, Copy)]
        struct Point { x: i32, y: i32 }

        pub(crate) fn main() {
            let c = 'Q';

            // 赋值语句中左边的 `ref` 关键字等价于右边的 `&` 符号。
            let ref ref_c1 = c;
            let ref_c2 = &c;

            println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

            let point = Point { x: 0, y: 0 };

            // 在解构一个结构体时 `ref` 同样有效。
            let _copy_of_x = {
                // `ref_to_x` 是一个指向 `point` 的 `x` 字段的引用。
                let Point { x: ref ref_to_x, y: _ } = point;

                // 返回一个 `point` 的 `x` 字段的拷贝。
                *ref_to_x
            };

            // `point` 的可变拷贝
            let mut mutable_point = point;

            {
                // `ref` 可以与 `mut` 结合以创建可变引用。
                let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

                // 通过可变引用来改变 `mutable_point` 的字段 `y`。
                *mut_ref_to_y = 1;
            }

            println!("point is ({}, {})", point.x, point.y);
            println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

            // 包含一个指针的可变元组
            let mut mutable_tuple = (Box::new(5u32), 3u32);

            {
                // 解构 `mutable_tuple` 来改变 `last` 的值。
                let (_, ref mut last) = mutable_tuple;
                *last = 2u32;
            }

            println!("tuple is {:?}", mutable_tuple);
        }

    }
    pub mod c15_4 {
        // 下面使用连线来标注各个变量的创建和销毁，从而显示出生命周期。
    // `i` 的生命周期最长，因为它的作用域完全覆盖了 `borrow1` 和
    // `borrow2` 的。`borrow1` 和 `borrow2` 的周期没有关联，
    // 因为它们各不相交。
        fn main() {
            let i = 3; // Lifetime for `i` starts. ────────────────┐
            //                                                     │
            { //                                                   │
                let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
                //                                                ││
                println!("borrow1: {}", borrow1); //              ││
            } // `borrow1 ends. ──────────────────────────────────┘│
            //                                                     │
            //                                                     │
            { //                                                   │
                let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
                //                                                ││
                println!("borrow2: {}", borrow2); //              ││
            } // `borrow2` ends. ─────────────────────────────────┘│
            //                                                     │
        }   // Lifetime ends. ─────────────────────────────────────┘

    }
    pub mod c15_4_1 {
        // `print_refs` 接受两个 `i32` 的引用，它们有不同的生命周期 `'a` 和 `'b`。
        // 这两个生命周期都必须至少要和 `print_refs` 函数一样长。
        fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
            println!("x is {} and y is {}", x, y);
        }

        // 不带参数的函数，不过有一个生命周期参数 `'a`。
        fn failed_borrow<'a>() {
            let _x = 12;

            // 报错：`_x` 的生命周期不够长
            // let y: &'a i32 = &_x;
            // 在函数内部使用生命周期 `'a` 作为显式类型标注将导致失败，因为 `&_x` 的
            // 生命周期比 `y` 的短。短生命周期不能强制转换成长生命周期。
        }

        pub fn main() {
            // 创建变量，稍后用于借用。
            let (four, nine) = (4, 9);

            // 两个变量的借用（`&`）都传进函数。
            print_refs(&four, &nine);
            // 任何被借用的输入量都必须比借用者生存得更长。
            // 也就是说，`four` 和 `nine` 的生命周期都必须比 `print_refs` 的长。

            failed_borrow();
            // `failed_borrow` 未包含引用，因此不要求 `'a` 长于函数的生命周期，
            // 但 `'a` 寿命确实更长。因为该生命周期从未被约束，所以默认为 `'static`。
        }
    }
    pub mod c15_4_9 {
        // `elided_input` 和 `annotated_input` 事实上拥有相同的签名，
        // `elided_input` 的生命周期会被编译器自动添加：
        fn elided_input(x: &i32) {
            println!("`elided_input`: {}", x)
        }

        fn annotated_input<'a>(x: &'a i32) {
            println!("`annotated_input`: {}", x)
        }

        // 类似地，`elided_pass` 和 `annotated_pass` 也拥有相同的签名，
        // 生命周期会被隐式地添加进 `elided_pass`：
        fn elided_pass(x: &i32) -> &i32 { x }

        fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

        pub(crate) fn main() {
            let x = 3;

            elided_input(&x);
            annotated_input(&x);

            println!("`elided_pass`: {}", elided_pass(&x));
            println!("`annotated_pass`: {}", annotated_pass(&x));
        }

    }

    pub mod c15_4_2 {
        // 一个拥有生命周期 `'a` 的输入引用，其中 `'a` 的存活时间
        // 至少与函数的一样长。
        fn print_one<'a>(x: &'a i32) {
            println!("`print_one`: x is {}", x);
        }

        // 可变引用同样也可能拥有生命周期。
        fn add_one<'a>(x: &'a mut i32) {
            *x += 1;
        }

        // 拥有不同生命周期的多个元素。对下面这种情形，两者即使拥有
        // 相同的生命周期 `'a` 也没问题，但对一些更复杂的情形，可能
        // 就需要不同的生命周期了。
        fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
            println!("`print_multi`: x is {}, y is {}", x, y);
        }

        // 返回传递进来的引用也是可行的。
        // 但必须返回正确的生命周期。
        fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

        // fn invalid_output<'a>() -> &'a String { &String::from("foo") }
        // 上面代码是无效的：`'a` 存活的时间必须比函数的长。
        // 这里的 `&String::from("foo")` 将会创建一个 `String` 类型，然后对它取引用。
        // 数据在离开作用域时删掉，返回一个指向无效数据的引用。

        pub fn main() {
            let x = 7;
            let y = 9;

            print_one(&x);
            print_multi(&x, &y);

            let z = pass_x(&x, &y);
            print_one(z);

            let mut t = 3;
            add_one(&mut t);
            print_one(&t);
        }

    }

    pub mod c15_4_3 {
        struct Owner(i32);

        impl Owner {
            // 标注生命周期，就像独立的函数一样。
            fn add_one<'a>(&'a mut self) { self.0 += 1; }
            fn print<'a>(&'a self) {
                println!("`print`: {}", self.0);
            }
        }

        pub fn main() {
            let mut owner  = Owner(18);

            owner.add_one();
            owner.print();
        }

    }

    pub mod c15_4_4 {
        // 一个 `Borrowed` 类型，含有一个指向 `i32` 类型的引用。
// 该引用必须比 `Borrowed` 寿命更长。
        #[derive(Debug)]
        struct Borrowed<'a>(&'a i32);

        // 和前面类似，这里的两个引用都必须比这个结构体长寿。
        #[derive(Debug)]
        struct NamedBorrowed<'a> {
            x: &'a i32,
            y: &'a i32,
        }

        // 一个枚举类型，其取值不是 `i32` 类型就是一个指向 `i32` 的引用。
        #[derive(Debug)]
        enum Either<'a> {
            Num(i32),
            Ref(&'a i32),
        }

        pub fn main() {
            let x = 18;
            let y = 15;

            let single = Borrowed(&x);
            let double = NamedBorrowed { x: &x, y: &y };
            let reference = Either::Ref(&x);
            let number    = Either::Num(y);

            println!("x is borrowed in {:?}", single);
            println!("x and y are borrowed in {:?}", double);
            println!("x is borrowed in {:?}", reference);
            println!("y is *not* borrowed in {:?}", number);
        }

    }

    pub mod c15_4_5 {
        // 带有生命周期标注的结构体。
        #[derive(Debug)]
        struct Borrowed<'a> {
            x: &'a i32,
        }

        // 给 impl 标注生命周期。
        impl<'a> Default for Borrowed<'a> {
            fn default() -> Self {
                Self {
                    x: &10,
                }
            }
        }

        pub fn main() {
            let b: Borrowed = Default::default();
            println!("b is {:?}", b);
        }

    }

    pub mod c15_4_6 {
        use std::fmt::Debug; // 用于约束的 trait。

        #[derive(Debug)]
        struct Ref<'a, T: 'a>(&'a T);
        // `Ref` 包含一个指向泛型类型 `T` 的引用，其中 `T` 拥有一个未知的生命周期
        // `'a`。`T` 拥有生命周期限制， `T` 中的任何*引用*都必须比 `'a` 活得更长。另外
        // `Ref` 的生命周期也不能超出 `'a`。

        // 一个泛型函数，使用 `Debug` trait 来打印内容。
        fn print<T>(t: T) where
            T: Debug {
            println!("`print`: t is {:?}", t);
        }

        // 这里接受一个指向 `T` 的引用，其中 `T` 实现了 `Debug` trait，并且在 `T` 中的
        // 所有*引用*都必须比 `'a'` 存活时间更长。另外，`'a` 也要比函数活得更长。
        fn print_ref<'a, T>(t: &'a T) where
            T: Debug + 'a {
            println!("`print_ref`: t is {:?}", t);
        }

        pub fn main() {
            let x = 7;
            let ref_x = Ref(&x);

            print_ref(&ref_x);
            print(ref_x);
        }

    }
    pub mod c15_4_7 {
        // 在这里，Rust 推导了一个尽可能短的生命周期。
        // 然后这两个引用都被强制转成这个生命周期。
        fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
            first * second
        }

        // `<'a: 'b, 'b>` 读作生命周期 `'a` 至少和 `'b` 一样长。
        // 在这里我们我们接受了一个 `&'a i32` 类型并返回一个 `&'b i32` 类型，这是
        // 强制转换得到的结果。
        fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
            first
        }

        pub(crate) fn main() {
            let first = 2; // 较长的生命周期

            {
                let second = 3; // 较短的生命周期

                println!("The product is {}", multiply(&first, &second));
                println!("{} is the first", choose_first(&first, &second));
            };
        }

    }

    pub mod c15_4_8 {
        // 产生一个拥有 `'static` 生命周期的常量。
        static NUM: i32 = 18;

        // 返回一个指向 `NUM` 的引用，该引用不取 `NUM` 的 `'static` 生命周期，
        // 而是被强制转换成和输入参数的一样。
        fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
            &NUM
        }

        pub(crate) fn main() {
            {
                // 产生一个 `string` 字面量并打印它：
                let static_string = "I'm in read-only memory";
                println!("static_string: {}", static_string);

                // 当 `static_string` 离开作用域时，该引用不能再使用，不过
                // 数据仍然存在于二进制文件里面。
            }

            {
                // 产生一个整型给 `coerce_static` 使用：
                let lifetime_num = 9;

                // 将对 `NUM` 的引用强制转换成 `lifetime_num` 的生命周期：
                let coerced_static = coerce_static(&lifetime_num);

                println!("coerced_static: {}", coerced_static);
            }

            println!("NUM: {} stays accessible!", NUM);
        }

    }
}

pub mod c16 {
    struct Sheep { naked: bool, name: &'static str }

    trait Animal {
        // 静态方法签名；`Self` 表示实现者类型（implementor type）。
        fn new(name: &'static str) -> Self;

        // 实例方法签名；这些方法将返回一个字符串。
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        // trait 可以提供默认的方法定义。
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                // 实现者可以使用它的 trait 方法。
                println!("{} is already naked...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }

    // 对 `Sheep` 实现 `Animal` trait。
    impl Animal for Sheep {
        // `Self` 是实现者类型：`Sheep`。
        fn new(name: &'static str) -> Sheep {
            Sheep { name: name, naked: false }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "baaaaah?"
            } else {
                "baaaaah!"
            }
        }

        // 默认 trait 方法可以重载。
        fn talk(&self) {
            // 例如我们可以增加一些安静的沉思。
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }

    pub fn main() {
        // 这种情况需要类型标注。
        let mut dolly: Sheep = Animal::new("Dolly");
        // 试一试 ^ 移除类型标注。

        dolly.talk();
        dolly.shear();
        dolly.talk();
    }

    pub mod c16_1 {
        // `Centimeters`，可以比较的元组结构体
        #[derive(PartialEq, PartialOrd)]
        struct Centimeters(f64);

        // `Inches`，可以打印的元组结构体
        #[derive(Debug)]
        struct Inches(i32);

        impl Inches {
            fn to_centimeters(&self) -> Centimeters {
                let &Inches(inches) = self;

                Centimeters(inches as f64 * 2.54)
            }
            // todo: Can I impl PartialEq here?
        }

        // `Seconds`，不带附加属性的元组结构体
        struct Seconds(i32);

        pub fn main() {
            let _one_second = Seconds(1);

            // 报错：`Seconds` 不能打印；它没有实现 `Debug` trait
            //println!("One second looks like: {:?}", _one_second);
            // 试一试 ^ 取消此行注释

            // 报错：`Seconds`不能比较；它没有实现 `PartialEq` trait
            //let _this_is_true = (_one_second == _one_second);
            // 试一试 ^ 取消此行注释

            let foot = Inches(12);

            println!("One foot equals {:?}", foot);

            let meter = Centimeters(100.0);

            let cmp =
                if foot.to_centimeters() < meter {
                    "smaller"
                } else {
                    "bigger"
                };

            println!("One foot is {} than one meter.", cmp);
        }

    }

    pub mod c16_2 {
        struct Sheep {}
        struct Cow {}

        trait Animal {
            // 实例方法签名
            fn noise(&self) -> &'static str;
        }

        // 实现 `Sheep` 的 `Animal` trait。
        impl Animal for Sheep {
            fn noise(&self) -> &'static str {
                "baaaaah!"
            }
        }

        // 实现 `Cow` 的 `Animal` trait。
        impl Animal for Cow {
            fn noise(&self) -> &'static str {
                "moooooo!"
            }
        }

        // 返回一些实现 Animal 的结构体，但是在编译时我们不知道哪个结构体。
        fn random_animal(random_number: f64) -> Box<dyn Animal> {
            if random_number < 0.5 {
                Box::new(Sheep {})
            } else {
                Box::new(Cow {})
            }
        }

        pub fn main() {
            let random_number = 0.234;
            let animal = random_animal(random_number);
            println!("You've randomly chosen an animal, and it says {}", animal.noise());
        }


    }

    pub mod c16_3 {
        use std::ops;

        struct Foo;
        struct Bar;

        #[derive(Debug)]
        struct FooBar;

        #[derive(Debug)]
        struct BarFoo;

        // `std::ops::Add` trait 用来指明 `+` 的功能，这里我们实现 `Add<Bar>`，它是用于
        // 把对象和 `Bar` 类型的右操作数（RHS）加起来的 `trait`。
        // 下面的代码块实现了 `Foo + Bar = FooBar` 这样的运算。
        impl ops::Add<Bar> for Foo {
            type Output = FooBar;

            fn add(self, _rhs: Bar) -> FooBar {
                println!("> Foo.add(Bar) was called");

                FooBar
            }
        }

        // 通过颠倒类型，我们实现了不服从交换律的加法。
        // 这里我们实现 `Add<Foo>`，它是用于把对象和 `Foo` 类型的右操作数加起来的 trait。
        // 下面的代码块实现了 `Bar + Foo = BarFoo` 这样的运算。
        impl ops::Add<Foo> for Bar {
            type Output = BarFoo;

            fn add(self, _rhs: Foo) -> BarFoo {
                println!("> Bar.add(Foo) was called");

                BarFoo
            }
        }

        pub(crate) fn main() {
            println!("Foo + Bar = {:?}", Foo + Bar);
            println!("Bar + Foo = {:?}", Bar + Foo);
        }

    }

    pub mod c16_4 {
        struct Droppable {
            name: &'static str,
        }

        // 这个简单的 `drop` 实现添加了打印到控制台的功能。
        impl Drop for Droppable {
            fn drop(&mut self) {
                println!("> Dropping {}", self.name);
            }
        }

        pub(crate) fn main() {
            let _a = Droppable { name: "a" };

            // 代码块 A
            {
                let _b = Droppable { name: "b" };

                // 代码块 B
                {
                    let _c = Droppable { name: "c" };
                    let _d = Droppable { name: "d" };

                    println!("Exiting block B");
                }
                println!("Just exited block B");

                println!("Exiting block A");
            }
            println!("Just exited block A");

            // 变量可以手动使用 `drop` 函数来销毁。
            drop(_a);
            // 试一试 ^ 将此行注释掉。

            println!("end of the main function");

            // `_a` *不会*在这里再次销毁，因为它已经被（手动）销毁。
        }

    }

    pub mod c16_5{
        struct Fibonacci {
            curr: u32,
            next: u32,
        }

        // 为 `Fibonacci`（斐波那契）实现 `Iterator`。
        // `Iterator` trait 只需定义一个能返回 `next`（下一个）元素的方法。
        impl Iterator for Fibonacci {
            type Item = u32;

            // 我们在这里使用 `.curr` 和 `.next` 来定义数列（sequence）。
            // 返回类型为 `Option<T>`：
            //     * 当 `Iterator` 结束时，返回 `None`。
            //     * 其他情况，返回被 `Some` 包裹（wrap）的下一个值。
            // todo: what is wrap? its effect?
            fn next(&mut self) -> Option<u32> {
                let new_next = self.curr + self.next;

                self.curr = self.next;
                self.next = new_next;

                // 既然斐波那契数列不存在终点，那么 `Iterator` 将不可能
                // 返回 `None`，而总是返回 `Some`。
                Some(self.curr)
            }
        }

        // 返回一个斐波那契数列生成器
        fn fibonacci() -> Fibonacci {
            Fibonacci { curr: 1, next: 1 }
        }

        pub fn main() {
            // `0..3` 是一个 `Iterator`，会产生：0、1 和 2。
            let mut sequence = 0..3;

            println!("Four consecutive `next` calls on 0..3");
            println!("> {:?}", sequence.next());
            println!("> {:?}", sequence.next());
            println!("> {:?}", sequence.next());
            println!("> {:?}", sequence.next());

            // `for` 遍历 `Iterator` 直到返回 `None`，
            // 并且每个 `Some` 值都被解包（unwrap），然后绑定给一个变量（这里是 `i`）。
            println!("Iterate through 0..3 using `for`");
            for i in 0..3 {
                println!("> {}", i);
            }

            // `take(n)` 方法提取 `Iterator` 的前 `n` 项。
            println!("The first four terms of the Fibonacci sequence are: ");
            for i in fibonacci().take(4) {
                println!("> {}", i);
            }

            // `skip(n)` 方法移除前 `n` 项，从而缩短了 `Iterator` 。
            println!("The next four terms of the Fibonacci sequence are: ");
            for i in fibonacci().skip(4).take(4) {
                println!("> {}", i);
            }

            let array = [1u32, 3, 3, 7];

            // `iter` 方法对数组/slice 产生一个 `Iterator`。
            println!("Iterate the following array {:?}", &array);
            for i in array.iter() {
                println!("> {}", i);
            }
        }

    }

    pub mod c16_6 {
        use std::iter;
        use std::vec::IntoIter;

        // 该函数组合了两个 `Vec <i32>` 并在其上返回一个迭代器。
        // 看看它的返回类型多么复杂！
        fn combine_vecs_explicit_return_type(
            v: Vec<i32>,
            u: Vec<i32>,
        ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
            v.into_iter().chain(u.into_iter()).cycle()
        }

        // 这是完全相同的函数，但其返回类型使用 `impl Trait`。
        // 看看它多么简单！
        fn combine_vecs(
            v: Vec<i32>,
            u: Vec<i32>,
        ) -> impl Iterator<Item=i32> {
            v.into_iter().chain(u.into_iter()).cycle()
        }

        pub fn main() {
            let v1 = vec![1, 2, 3];
            let v2 = vec![4, 5];
            let mut v3 = combine_vecs(v1, v2);
            assert_eq!(Some(1), v3.next());
            assert_eq!(Some(2), v3.next());
            assert_eq!(Some(3), v3.next());
            assert_eq!(Some(4), v3.next());
            assert_eq!(Some(5), v3.next());
            println!("all done");
        }

        // 返回一个将输入和 `y` 相加的函数
        fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
            let closure = move |x: i32| { x + y };
            closure
        }

        pub fn main2() {
            let plus_one = make_adder_function(1);
            assert_eq!(plus_one(2), 3);
        }

        fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
            numbers
                .iter()
                .filter(|x| x > &&0)
                .map(|x| x * 2)
        }
    }

    pub mod c16_7 {
        // 不含资源的单元结构体
        #[derive(Debug, Clone, Copy)]
        struct Nil;

        // 一个包含资源的结构体，它实现了 `Clone` trait
        #[derive(Clone, Debug)]
        struct Pair(Box<i32>, Box<i32>);

        pub(crate) fn main() {
            // 实例化 `Nil`
            let nil = Nil;
            // 复制 `Nil`，没有资源用于移动（move）
            let copied_nil = nil;

            // 两个 `Nil` 都可以独立使用
            println!("original: {:?}", nil);
            println!("copy: {:?}", copied_nil);

            // 实例化 `Pair`
            let pair = Pair(Box::new(1), Box::new(2));
            println!("original: {:?}", pair);

            // 将 `pair` 绑定到 `moved_pair`，移动（move）了资源
            let moved_pair = pair;
            println!("copy: {:?}", moved_pair);

            // 报错！`pair` 已失去了它的资源。
            //println!("original: {:?}", pair);
            // 试一试 ^ 取消此行注释。

            // 将 `moved_pair`（包括其资源）克隆到 `cloned_pair`。
            let cloned_pair = moved_pair.clone();
            // 使用 std::mem::drop 来销毁原始的 pair。
            drop(moved_pair);

            // 报错！`moved_pair` 已被销毁。
            //println!("copy: {:?}", moved_pair);
            // 试一试 ^ 将此行注释掉。

            // 由 .clone() 得来的结果仍然可用！
            println!("clone: {:?}", cloned_pair);
        }
    }
    pub mod c16_8 {
        trait Person {
            fn name(&self) -> String;
        }

        // Person 是 Student 的父 trait。
        // 实现 Student 需要你也 impl 了 Person。
        trait Student: Person {
            fn university(&self) -> String;
        }

        trait Programmer {
            fn fav_language(&self) -> String;
        }

        // CompSciStudent (computer science student，计算机科学的学生) 是 Programmer 和 Student 两者的子类。
        // 实现 CompSciStudent 需要你同时 impl 了两个父 trait。
        trait CompSciStudent: Programmer + Student {
            fn git_username(&self) -> String;
        }

        fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
            format!(
                "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
                student.name(),
                student.university(),
                student.fav_language(),
                student.git_username()
            )
        }

        fn main() {}

    }
    pub mod c16_9 {
        trait UsernameWidget {
            // 从这个 widget 中获取选定的用户名
            fn get(&self) -> String;
        }

        trait AgeWidget {
            // 从这个 widget 中获取选定的年龄
            fn get(&self) -> u8;
        }

        // 同时具有 UsernameWidget 和 AgeWidget 的表单
        struct Form {
            username: String,
            age: u8,
        }

        impl UsernameWidget for Form {
            fn get(&self) -> String {
                self.username.clone()
            }
        }

        impl AgeWidget for Form {
            fn get(&self) -> u8 {
                self.age
            }
        }

        pub(crate) fn main() {
            let form = Form{
                username: "rustacean".to_owned(),
                age: 28,
            };

            // 如果取消注释此行，则会收到一条错误消息，提示 “multiple `get` found”（找到了多个`get`）。
            // 因为毕竟有多个名为 `get` 的方法。
            // println!("{}", form.get());

            let username = <Form as UsernameWidget>::get(&form);
            assert_eq!("rustacean".to_owned(), username);
            let age = <Form as AgeWidget>::get(&form);
            assert_eq!(28, age);
        }

    }
}

pub mod c17 {
    // 这是一个简单的宏，名为 `say_hello`。
    macro_rules! say_hello {
    // `()` 表示此宏不接受任何参数。
    () => (
        // 此宏将会展开成这个代码块里面的内容。
        println!("Hello!");
    )
}

    pub fn main() {
        // 这个调用将会展开成 `println("Hello");`!
        say_hello!()
    }


    pub mod c17_1_1 {
        macro_rules! create_function {
            // 此宏接受一个 `ident` 指示符表示的参数，并创建一个名为 `$func_name` 的函数。
            // `ident` 指示符用于变量名或函数名
            ($func_name:ident) => (
                fn $func_name() {
                    // `stringify!` 宏把 `ident` 转换成字符串。
                    println!("You called {:?}()",
                             stringify!($func_name))
                }
            )
        }

        // 借助上述宏来创建名为 `foo` 和 `bar` 的函数。
        create_function!(foo);
        create_function!(bar);

        macro_rules! print_result {
            // 此宏接受一个 `expr` 类型的表达式，并将它作为字符串，连同其结果一起
            // 打印出来。
            // `expr` 指示符表示表达式。
            ($expression:expr) => (
                // `stringify!` 把表达式*原样*转换成一个字符串。
                println!("{:?} = {:?}",
                         stringify!($expression),
                         $expression)
            )
        }

        pub fn main() {
            foo();
            bar();

            print_result!(1u32 + 1);

            // 回想一下，代码块也是表达式！
            print_result!({
                let x = 1u32;

                x * x + 2 * x - 1
            });
        }

    }
    pub mod c17_1_2 {
        // 根据你调用它的方式，`test!` 将以不同的方式来比较 `$left` 和 `$right`。
        macro_rules! test {
            // 参数不需要使用逗号隔开。
            // 参数可以任意组合！
            ($left:expr; and $right:expr) => (
                println!("{:?} and {:?} is {:?}",
                         stringify!($left),
                         stringify!($right),
                         $left && $right)
            );
            // ^ 每个分支都必须以分号结束。
            ($left:expr; or $right:expr) => (
                println!("{:?} or {:?} is {:?}",
                         stringify!($left),
                         stringify!($right),
                         $left || $right)
            );
        }

        pub fn main() {
            test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
            test!(true; or false);
        }

    }
    pub mod c17_1_3 {
        // `min!` 将求出任意数量的参数的最小值。
        macro_rules! find_min {
            // 基本情形：
            ($x:expr) => ($x);
            // `$x` 后面跟着至少一个 `$y,`
            ($x:expr, $($y:expr),+) => (
                // 对 `$x` 后面的 `$y` 们调用 `find_min!`
                std::cmp::min($x, find_min!($($y),+))
            )
        }

        pub fn main() {
            println!("{}", find_min!(1u32));
            println!("{}", find_min!(1u32 + 2 , 2u32));
            println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
        }

    }

    pub mod c17_2 {
        use std::ops::{Add, Mul, Sub};

        macro_rules! assert_equal_len {
            // `tt`（token tree，标记树）指示符表示运算符和标记。
            ($a:ident, $b: ident, $func:ident, $op:tt) => (
                assert!($a.len() == $b.len(),
                        "{:?}: dimension mismatch: {:?} {:?} {:?}",
                        stringify!($func),
                        ($a.len(),),
                        stringify!($op),
                        ($b.len(),));
            )
        }

        macro_rules! op {
            ($func:ident, $bound:ident, $op:tt, $method:ident) => (
                fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
                    assert_equal_len!(xs, ys, $func, $op);

                    for (x, y) in xs.iter_mut().zip(ys.iter()) {
                        *x = $bound::$method(*x, *y);
                        // *x = x.$method(*y);
                    }
                }
            )
        }

        // 实现 `add_assign`、`mul_assign` 和 `sub_assign` 等函数。
        op!(add_assign, Add, +=, add);
        op!(mul_assign, Mul, *=, mul);
        op!(sub_assign, Sub, -=, sub);

        mod test {
            use std::iter;
            macro_rules! test {
                ($func: ident, $x:expr, $y:expr, $z:expr) => {
                    #[test]
                    fn $func() {
                        for size in 0usize..10 {
                            let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                            let y: Vec<_> = iter::repeat($y).take(size).collect();
                            let z: Vec<_> = iter::repeat($z).take(size).collect();

                            super::$func(&mut x, &y);

                            assert_eq!(x, z);
                        }
                    }
                }
            }

            // 测试 `add_assign`、`mul_assign` 和 `sub_assign`
            test!(add_assign, 1u32, 2u32, 3u32);
            test!(mul_assign, 2u32, 3u32, 6u32);
            test!(sub_assign, 3u32, 2u32, 1u32);
        }
    }
    pub mod c17_3 {
        macro_rules! calculate {
            (eval $e:expr) => {{
                {
                    let val: usize = $e; // 强制类型为整型
                    println!("{} = {}", stringify!{$e}, val);
                }
            }};
        }

        fn main() {
            calculate! {
                eval 1 + 2 // 看到了吧，`eval` 可并不是 Rust 的关键字！
            }

            calculate! {
                eval (1 + 2) * (3 / 4)
            }
        }

    }
    pub mod c17_4 {
        macro_rules! calculate {
            // 单个 `eval` 的模式
            (eval $e:expr) => {{
                {
                    let val: usize = $e; // Force types to be integers
                    println!("{} = {}", stringify!{$e}, val);
                }
            }};

            // 递归地拆解多重的 `eval`
            (eval $e:expr, $(eval $es:expr),+) => {{
                    calculate! { eval $e }
                    calculate! { $(eval $es),+ }
                }};
            }

        pub fn main() {
            calculate! { // 妈妈快看，可变参数的 `calculate!`！
                eval 1 + 2,
                eval 3 + 4,
                eval (2 * 3) + 1
            }
        }

    }

}

pub mod c18 {
    pub mod c18_1 {
        fn give_princess(gift: &str) {
            // 公主讨厌蛇，所以如果公主表示厌恶的话我们要停止！
            if gift == "snake" { panic!("AAAaaaaa!!!!"); }

            println!("I love {}s!!!!!", gift);
        }

        pub(crate) fn main() {
            give_princess("teddy bear");
            give_princess("snake");
        }

    }

    pub mod c18_2 {
        // 平民（commoner）们见多识广，收到什么礼物都能应对。
        // 所有礼物都显式地使用 `match` 来处理。
        fn give_commoner(gift: Option<&str>) {
            // 指出每种情况下的做法。
            match gift {
                Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
                Some(inner)   => println!("{}? How nice.", inner),
                None          => println!("No gift? Oh well."),
            }
        }

        // 养在深闺人未识的公主见到蛇就会 `panic`（恐慌）。
        // 这里所有的礼物都使用 `unwrap` 隐式地处理。
        fn give_princess(gift: Option<&str>) {
            // `unwrap` 在接收到 `None` 时将返回 `panic`。
            let inside = gift.unwrap();
            if inside == "snake" { panic!("AAAaaaaa!!!!"); }

            println!("I love {}s!!!!!", inside);
        }

        pub(crate) fn main() {
            let food  = Some("chicken");
            let snake = Some("snake");
            let void  = None;

            give_commoner(food);
            give_commoner(snake);
            give_commoner(void);

            let bird = Some("robin");
            let nothing = None;

            give_princess(bird);
            give_princess(nothing);
        }

    }

    pub mod c18_2_1 {
        fn next_birthday(current_age: Option<u8>) -> Option<String> {
            // 如果 `current_age` 是 `None`，这将返回 `None`。
            // 如果 `current_age` 是 `Some`，内部的 `u8` 将赋值给 `next_age`。
            let next_age: u8 = current_age?;
            Some(format!("Next year I will be {}", next_age))
        }

        struct Person {
            job: Option<Job>,
        }

        #[derive(Clone, Copy)]
        struct Job {
            phone_number: Option<PhoneNumber>,
        }

        #[derive(Clone, Copy)]
        struct PhoneNumber {
            area_code: Option<u8>,
            number: u32,
        }

        impl Person {

            // 获取此人的工作电话号码的区号（如果存在的话）。
            fn work_phone_area_code(&self) -> Option<u8> {
                // 没有`？`运算符的话，这将需要很多的嵌套的 `match` 语句。
                // 这将需要更多代码——尝试自己编写一下，看看哪个更容易。
                self.job?.phone_number?.area_code
            }
        }

        pub(crate) fn main() {
            let p = Person {
                job: Some(Job {
                    phone_number: Some(PhoneNumber {
                        area_code: Some(61),
                        number: 439222222,
                    }),
                }),
            };

            assert_eq!(p.work_phone_area_code(), Some(61));
        }

    }
    pub mod c18_2_2 {
        #![allow(dead_code)]

        #[derive(Debug)] enum Food { Apple, Carrot, Potato }

        #[derive(Debug)] struct Peeled(Food);
        #[derive(Debug)] struct Chopped(Food);
        #[derive(Debug)] struct Cooked(Food);

        // 削皮。如果没有食物，就返回 `None`。否则返回削好皮的食物。
        fn peel(food: Option<Food>) -> Option<Peeled> {
            match food {
                Some(food) => Some(Peeled(food)),
                None       => None,
            }
        }

        // 切食物。如果没有食物，就返回 `None`。否则返回切好的食物。
        fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
            match peeled {
                Some(Peeled(food)) => Some(Chopped(food)),
                None               => None,
            }
        }

        // 烹饪食物。这里，我们使用 `map()` 来替代 `match` 以处理各种情况。
        fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
            chopped.map(|Chopped(food)| Cooked(food))
        }

        // 这个函数会完成削皮切块烹饪一条龙。我们把 `map()` 串起来，以简化代码。
        fn process(food: Option<Food>) -> Option<Cooked> {
            food.map(|f| Peeled(f))
                .map(|Peeled(f)| Chopped(f))
                .map(|Chopped(f)| Cooked(f))
        }

        // 在尝试吃食物之前确认食物是否存在是非常重要的！
        fn eat(food: Option<Cooked>) {
            match food {
                Some(food) => println!("Mmm. I love {:?}", food),
                None       => println!("Oh no! It wasn't edible."),
            }
        }

        pub(crate) fn main() {
            let apple = Some(Food::Apple);
            let carrot = Some(Food::Carrot);
            let potato = None;

            let cooked_apple = cook(chop(peel(apple)));
            let cooked_carrot = cook(chop(peel(carrot)));

            // 现在让我们试试看起来更简单的 `process()`。
            let cooked_potato = process(potato);

            eat(cooked_apple);
            eat(cooked_carrot);
            eat(cooked_potato);
        }
    }
    pub mod c18_2_3 {
        #![allow(dead_code)]

        #[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
        #[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

        // 我们没有制作寿司所需的原材料（ingredient）（有其他的原材料）。
        fn have_ingredients(food: Food) -> Option<Food> {
            match food {
                Food::Sushi => None,
                _           => Some(food),
            }
        }

        // 我们拥有全部食物的食谱，除了法国蓝带猪排（Cordon Bleu）的。
        fn have_recipe(food: Food) -> Option<Food> {
            match food {
                Food::CordonBleu => None,
                _                => Some(food),
            }
        }


        // 要做一份好菜，我们需要原材料和食谱。
        // 我们可以借助一系列 `match` 来表达这个逻辑：
        fn cookable_v1(food: Food) -> Option<Food> {
            match have_ingredients(food) {
                None       => None,
                Some(food) => match have_recipe(food) {
                    None       => None,
                    Some(food) => Some(food),
                },
            }
        }

        // 也可以使用 `and_then()` 把上面的逻辑改写得更紧凑：
        fn cookable_v2(food: Food) -> Option<Food> {
            have_ingredients(food).and_then(have_recipe)
        }

        fn eat(food: Food, day: Day) {
            match cookable_v2(food) {
                Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
                None       => println!("Oh no. We don't get to eat on {:?}?", day),
            }
        }

        pub(crate) fn main() {
            let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

            eat(cordon_bleu, Day::Monday);
            eat(steak, Day::Tuesday);
            eat(sushi, Day::Wednesday);
        }

    }

    pub mod c18_3 {
        fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
            // 我们试着用 `unwrap()` 把数字放出来。它会咬我们一口吗？
            let first_number = first_number_str.parse::<i32>().unwrap();
            let second_number = second_number_str.parse::<i32>().unwrap();
            first_number * second_number
        }

        pub fn main() {
            let twenty = multiply("10", "2");
            println!("double is {}", twenty);

            let tt = multiply("t", "2");
            println!("double is {}", tt);
        }

    }

    pub mod c18_3_1_old {
        use std::num::ParseIntError;

        // 修改了上一节中的返回类型，现在使用模式匹配而不是 `unwrap()`。
        fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
            match first_number_str.parse::<i32>() {
                Ok(first_number)  => {
                    match second_number_str.parse::<i32>() {
                        Ok(second_number)  => {
                            Ok(first_number * second_number)
                        },
                        Err(e) => Err(e),
                    }
                },
                Err(e) => Err(e),
            }
        }

        fn print(result: Result<i32, ParseIntError>) {
            match result {
                Ok(n)  => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        pub fn main() {
            // 这种情形下仍然会给出正确的答案。
            let twenty = multiply("10", "2");
            print(twenty);

            // 这种情况下就会提供一条更有用的错误信息。
            let tt = multiply("t", "2");
            print(tt);
        }

    }
    pub mod c18_3_1_new {
        use std::num::ParseIntError;

        // 就像 `Option` 那样，我们可以使用 `map()` 之类的组合算子。
        // 除去写法外，这个函数与上面那个完全一致，它的作用是：
        // 如果值是合法的，计算其乘积，否则返回错误。
        fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
            first_number_str.parse::<i32>().and_then(|first_number| {
                second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
            })
        }

        fn print(result: Result<i32, ParseIntError>) {
            match result {
                Ok(n)  => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        pub fn main() {
            // 这种情况下仍然会给出正确的答案。
            let twenty = multiply("10", "2");
            print(twenty);

            // 这种情况下就会提供一条更有用的错误信息。
            let tt = multiply("t", "2");
            print(tt);
        }

    }

    pub mod c18_3_2 {
        use std::num::ParseIntError;

        // 为带有错误类型 `ParseIntError` 的 `Result` 定义一个泛型别名。
        type AliasedResult<T> = Result<T, ParseIntError>;

        // 使用上面定义过的别名来表示上一节中的 `Result<i32,ParseIntError>` 类型。
        fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
            first_number_str.parse::<i32>().and_then(|first_number| {
                second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
            })
        }

        // 在这里使用别名又让我们节省了一些代码量。
        fn print(result: AliasedResult<i32>) {
            match result {
                Ok(n)  => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        pub fn main() {
            print(multiply("10", "2"));
            print(multiply("t", "2"));
        }

    }
    pub mod c18_3_3 {
        use std::num::ParseIntError;

        fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
            let first_number = match first_number_str.parse::<i32>() {
                Ok(first_number)  => first_number,
                Err(e) => return Err(e),
            };

            let second_number = match second_number_str.parse::<i32>() {
                Ok(second_number)  => second_number,
                Err(e) => return Err(e),
            };

            Ok(first_number * second_number)
        }

        fn print(result: Result<i32, ParseIntError>) {
            match result {
                Ok(n)  => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        pub fn main() {
            print(multiply("10", "2"));
            print(multiply("t", "2"));
        }

    }
    pub mod c18_3_4 {
        use std::num::ParseIntError;

        fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
            let first_number = first_number_str.parse::<i32>()?;
            let second_number = second_number_str.parse::<i32>()?;

            Ok(first_number * second_number)
        }

        fn print(result: Result<i32, ParseIntError>) {
            match result {
                Ok(n)  => println!("n is {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }

        pub fn main() {
            print(multiply("10", "2"));
            print(multiply("t", "2"));
        }

    }
    // todo: 18.4
}

pub mod c19 {
    pub mod c19_1 {
        use std::mem;

        #[allow(dead_code)]
        #[derive(Debug, Clone, Copy)]
        struct Point {
            x: f64,
            y: f64,
        }

        #[allow(dead_code)]
        struct Rectangle {
            p1: Point,
            p2: Point,
        }

        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        fn boxed_origin() -> Box<Point> {
            // 在堆上分配这个点（point），并返回一个指向它的指针
            Box::new(Point { x: 0.0, y: 0.0 })
        }

        pub(crate) fn main() {
            // （所有的类型标注都不是必需的）
            // 栈分配的变量
            let point: Point = origin();
            let rectangle: Rectangle = Rectangle {
                p1: origin(),
                p2: Point { x: 3.0, y: 4.0 }
            };

            // 堆分配的 rectangle（矩形）
            let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
                p1: origin(),
                p2: origin()
            });

            // 函数的输出可以装箱
            let boxed_point: Box<Point> = Box::new(origin());

            // 两层装箱
            let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

            println!("Point occupies {} bytes in the stack",
                     mem::size_of_val(&point));
            println!("Rectangle occupies {} bytes in the stack",
                     mem::size_of_val(&rectangle));

            // box 的宽度就是指针宽度
            println!("Boxed point occupies {} bytes in the stack",
                     mem::size_of_val(&boxed_point));
            println!("Boxed rectangle occupies {} bytes in the stack",
                     mem::size_of_val(&boxed_rectangle));
            println!("Boxed box occupies {} bytes in the stack",
                     mem::size_of_val(&box_in_a_box));

            // 将包含在 `boxed_point` 中的数据复制到 `unboxed_point`
            let unboxed_point: Point = *boxed_point;
            println!("Unboxed point occupies {} bytes in the stack",
                     mem::size_of_val(&unboxed_point));
        }

    }

    pub mod c19_2 {
        pub(crate) fn main() {
            // 迭代器可以被收集到 vector 中
            let collected_iterator: Vec<i32> = (0..10).collect();
            println!("Collected (0..10) into: {:?}", collected_iterator);

            // `vec!` 宏可用来初始化一个 vector
            let mut xs = vec![1i32, 2, 3];
            println!("Initial vector: {:?}", xs);

            // 在 vector 的尾部插入一个新的元素
            println!("Push 4 into the vector");
            xs.push(4);
            println!("Vector: {:?}", xs);

            // 报错！不可变 vector 不可增长
            // collected_iterator.push(0);
            // 改正 ^ 将此行注释掉

            // `len` 方法获得一个 vector 的当前大小
            println!("Vector size: {}", xs.len());

            // 下标使用中括号表示（从 0 开始）
            println!("Second element: {}", xs[1]);

            // `pop` 移除 vector 的最后一个元素并将它返回
            println!("Pop last element: {:?}", xs.pop());

            // 超出下标范围将抛出一个 panic
            // println!("Fourth element: {}", xs[3]);
            // 改正 ^ 注释掉此行

            // 迭代一个 `Vector` 很容易
            println!("Contents of xs:");
            for x in xs.iter() {
                println!("> {}", x);
            }

            // 可以在迭代 `Vector` 的同时，使用独立变量（`i`）来记录迭代次数
            for (i, x) in xs.iter().enumerate() {
                println!("In position {} we have value {}", i, x);
            }

            // 多亏了 `iter_mut`，可变的 `Vector` 在迭代的同时，其中每个值都能被修改
            for x in xs.iter_mut() {
                *x *= 3;
            }
            println!("Updated vector: {:?}", xs);
        }
    }

    pub mod c19_3 {
        pub(crate) fn main() {
            // （所有的类型标注都不是必需的）
            // 一个对只读内存中分配的字符串的引用
            let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
            println!("Pangram: {}", pangram);

            // 逆序迭代单词，这里并没有分配新的字符串
            println!("Words in reverse");
            for word in pangram.split_whitespace().rev() {
                println!("> {}", word);
            }

            // 复制字符到一个 vector，排序并移除重复值
            let mut chars: Vec<char> = pangram.chars().collect();
            chars.sort();
            chars.dedup();

            // 创建一个空的且可增长的 `String`
            let mut string = String::new();
            for c in chars {
                // 在字符串的尾部插入一个字符
                string.push(c);
                // 在字符串尾部插入一个字符串
                string.push_str(", ");
            }
            println!("{}", string);

            // 这个缩短的字符串是原字符串的一个切片，所以没有执行新的分配操作
            let chars_to_trim: &[char] = &[' ', ','];
            let trimmed_str: &str = string.trim_matches(chars_to_trim);
            println!("Used characters: {}", trimmed_str);

            // 堆分配一个字符串
            let alice = String::from("I like dogs");
            // 分配新内存并存储修改过的字符串
            let bob: String = alice.replace("dog", "cat");

            println!("Alice says: {}", alice);
            println!("Bob says: {}", bob);
        }

    }

    pub mod c19_4 {
        // 不会 `panic!` 的整数除法。
        fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
            if divisor == 0 {
                // 失败表示成 `None` 取值
                None
            } else {
                // 结果 Result 被包装到 `Some` 取值中
                Some(dividend / divisor)
            }
        }

        // 此函数处理可能失败的除法
        fn try_division(dividend: i32, divisor: i32) {
            // `Option` 值可以进行模式匹配，就和其他枚举类型一样
            match checked_division(dividend, divisor) {
                None => println!("{} / {} failed!", dividend, divisor),
                Some(quotient) => {
                    println!("{} / {} = {}", dividend, divisor, quotient)
                },
            }
        }

        pub fn main() {
            try_division(4, 2);
            try_division(1, 0);

            // 绑定 `None` 到一个变量需要类型标注
            let none: Option<i32> = None;
            let _equivalent_none = None::<i32>;

            let optional_float = Some(0f32);

            // 解包 `Some` 将取出被包装的值。
            println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

            // 解包 `None` 将会引发 `panic!`。
            // println!("{:?} unwraps to {:?}", none, none.unwrap());
        }

    }

    pub mod c19_5 {
        mod checked {
            // 我们想要捕获的数学 “错误”
            #[derive(Debug)]
            pub enum MathError {
                DivisionByZero,
                NegativeLogarithm,
                NegativeSquareRoot,
            }

            pub type MathResult = Result<f64, MathError>;

            pub fn div(x: f64, y: f64) -> MathResult {
                if y == 0.0 {
                    // 此操作将会失败，那么（与其让程序崩溃）不如把失败的原因包装在
                    // `Err` 中并返回
                    Err(MathError::DivisionByZero)
                } else {
                    // 此操作是有效的，返回包装在 `Ok` 中的结果
                    Ok(x / y)
                }
            }

            pub fn sqrt(x: f64) -> MathResult {
                if x < 0.0 {
                    Err(MathError::NegativeSquareRoot)
                } else {
                    Ok(x.sqrt())
                }
            }

            pub fn ln(x: f64) -> MathResult {
                if x < 0.0 {
                    Err(MathError::NegativeLogarithm)
                } else {
                    Ok(x.ln())
                }
            }
        }

        // `op(x, y)` === `sqrt(ln(x / y))`
        fn op(x: f64, y: f64) -> f64 {
            // 这是一个三层的 match 金字塔！
            match checked::div(x, y) {
                Err(why) => panic!("{:?}", why),
                Ok(ratio) => match checked::ln(ratio) {
                    Err(why) => panic!("{:?}", why),
                    Ok(ln) => match checked::sqrt(ln) {
                        Err(why) => panic!("{:?}", why),
                        Ok(sqrt) => sqrt,
                    },
                },
            }
        }

        pub fn main() {
            // 这会失败吗？
            println!("{}", op(1.0, 10.0));
        }

    }

    pub mod c19_6 {
        // 整型除法（/）的重新实现
        fn division(dividend: i32, divisor: i32) -> i32 {
            if divisor == 0 {
                // 除以 0 会引发 panic
                panic!("division by zero");
            } else {
                dividend / divisor
            }
        }

        // `main` 任务
        pub fn main() {
            // 堆分配的整数
            let _x = Box::new(0i32);

            // 此操作将会引发一个任务失败
            division(3, 0);

            println!("This point won't be reached!");

            // `_x` 应当会在此处被销毁
        }

    }

    pub mod c19_7 {
        use std::collections::HashMap;

        fn call(number: &str) -> &str {
            match number {
                "798-1364" => "We're sorry, the call cannot be completed as dialed.
            Please hang up and try again.",
                "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?",
                _ => "Hi! Who is this again?"
            }
        }

        pub fn main() {
            let mut contacts = HashMap::new();

            contacts.insert("Daniel", "798-1364");
            contacts.insert("Ashley", "645-7689");
            contacts.insert("Katie", "435-8291");
            contacts.insert("Robert", "956-1745");

            // 接受一个引用并返回 Option<&V>
            match contacts.get(&"Daniel") {
                Some(&number) => println!("Calling Daniel: {}", call(number)),
                _ => println!("Don't have Daniel's number."),
            }

            // 如果被插入的值为新内容，那么 `HashMap::insert()` 返回 `None`，
            // 否则返回 `Some(value)`
            contacts.insert("Daniel", "164-6743");

            match contacts.get(&"Ashley") {
                Some(&number) => println!("Calling Ashley: {}", call(number)),
                _ => println!("Don't have Ashley's number."),
            }

            contacts.remove(&("Ashley"));

            // `HashMap::iter()` 返回一个迭代器，该迭代器以任意顺序举出
            // (&'a key, &'a value) 对。
            for (contact, &number) in contacts.iter() {
                println!("Calling {}: {}", contact, call(number));
            }
        }

    }

    pub mod c19_7_1 {
        use std::collections::HashMap;

        // Eq 要求你对此类型推导 PartialEq。
        #[derive(PartialEq, Eq, Hash)]
        struct Account<'a>{
            username: &'a str,
            password: &'a str,
        }

        struct AccountInfo<'a>{
            name: &'a str,
            email: &'a str,
        }

        type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

        fn try_logon<'a>(accounts: &Accounts<'a>,
                         username: &'a str, password: &'a str){
            println!("Username: {}", username);
            println!("Password: {}", password);
            println!("Attempting logon...");

            let logon = Account {
                username,
                password,
            };

            match accounts.get(&logon) {
                Some(account_info) => {
                    println!("Successful logon!");
                    println!("Name: {}", account_info.name);
                    println!("Email: {}", account_info.email);
                },
                _ => println!("Login failed!"),
            }
        }

        pub fn main(){
            let mut accounts: Accounts = HashMap::new();

            let account = Account {
                username: "j.everyman",
                password: "password123",
            };

            let account_info = AccountInfo {
                name: "John Everyman",
                email: "j.everyman@email.com",
            };

            accounts.insert(account, account_info);

            try_logon(&accounts, "j.everyman", "psasword123");

            try_logon(&accounts, "j.everyman", "password123");
        }

    }

    pub mod c19_7_2 {
        use std::collections::HashSet;

        pub fn main() {
            let mut a: HashSet<i32> = vec!(1i32, 2, 3).into_iter().collect();
            let mut b: HashSet<i32> = vec!(2i32, 3, 4).into_iter().collect();

            assert!(a.insert(4));
            assert!(a.contains(&4));

            // 如果值已经存在，那么 `HashSet::insert()` 返回 false。
            // assert!(b.insert(4), "Value 4 is already in set B!");
            // 改正 ^ 将此行注释掉。

            b.insert(5);

            // 若一个集合（collection）的元素类型实现了 `Debug`，那么该集合也就实现了 `Debug`。
            // 这通常将元素打印成这样的格式 `[elem1, elem2, ...]
            println!("A: {:?}", a);
            println!("B: {:?}", b);

            // 乱序打印 [1, 2, 3, 4, 5]。
            println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

            // 这将会打印出 [1]
            println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

            // 乱序打印 [2, 3, 4]。
            println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

            // 打印 [1, 5]
            println!("Symmetric Difference: {:?}",
                     a.symmetric_difference(&b).collect::<Vec<&i32>>());
        }

    }

    pub mod c19_8 {
        use std::rc::Rc;

        pub fn main() {
            let rc_examples = "Rc examples".to_string();
            {
                println!("--- rc_a is created ---");

                let rc_a: Rc<String> = Rc::new(rc_examples);
                println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

                {
                    println!("--- rc_a is cloned to rc_b ---");

                    let rc_b: Rc<String> = Rc::clone(&rc_a);
                    println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
                    println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

                    // 如果两者内部的值相等的话，则两个 `Rc` 相等。
                    println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

                    // 我们可以直接使用值的方法
                    println!("Length of the value inside rc_a: {}", rc_a.len());
                    println!("Value of rc_b: {}", rc_b);

                    println!("--- rc_b is dropped out of scope ---");
                }

                println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

                println!("--- rc_a is dropped out of scope ---");
            }

            // 报错！`rc_examples` 已经移入 `rc_a`。
            // 而且当 `rc_a` 被删时，`rc_examples` 也被一起删除。
            // println!("rc_examples: {}", rc_examples);
            // 试一试 ^ 注释掉此行代码
        }

    }
    pub mod c19_9 {
        use std::sync::Arc;
        use std::thread;

        pub fn main() {
            // 这个变量声明用来指定其值的地方。
            let apple = Arc::new("the same apple");

            for _ in 0..10 {
                // 这里没有数值说明，因为它是一个指向内存堆中引用的指针。
                let apple = Arc::clone(&apple);

                thread::spawn(move || {
                    // 由于使用了Arc，线程可以使用分配在 `Arc` 变量指针位置的值来生成。
                    println!("{:?}", apple);
                });
            }
        }


    }
}

pub mod c20 {
    pub mod c20_1 {
        use std::thread;

        static NTHREADS: i32 = 10;

        // 这是主（`main`）线程
        pub fn main() {
            // 提供一个 vector 来存放所创建的子线程（children）。
            let mut children = vec![];

            for i in 0..NTHREADS {
                // 启动（spin up）另一个线程
                children.push(thread::spawn(move || {
                    println!("this is thread number {}", i)
                }));
            }

            for child in children {
                // 等待线程结束。返回一个结果。
                let _ = child.join();
            }
        }

    }
    pub mod c20_1_1 {
        use std::thread;

        // todo: THREAD_NUMBER 10
        // 这是 `main` 线程
        pub fn main() {
            // 这是我们要处理的数据。
            // 我们会通过线程实现 map-reduce 算法，从而计算每一位的和
            // 每个用空白符隔开的块都会分配给单独的线程来处理
            //
            // 试一试：插入空格，看看输出会怎样变化！
            let data = "86967897737416471853297327050364959
                        11861322575564723963297542624962850
                        70856234701860851907960690014725639
                        38397966707106094172783238747669219
                        523807952578  88236525459303330302837
                        58495327135744041048897885734297812
                        699202164389808735488 08413720956532
                        16278424637452589860345374828574668";

            // 创建一个向量，用于储存将要创建的子线程
            let mut children = vec![];

            /*************************************************************************
             * "Map" 阶段
             *
             * 把数据分段，并进行初始化处理
             ************************************************************************/

            // 把数据分段，每段将会单独计算
            // 每段都是完整数据的一个引用（&str）
            let chunked_data = data.split_whitespace();

            // 对分段的数据进行迭代。
            // .enumerate() 会把当前的迭代计数与被迭代的元素以元组 (index, element)
            // 的形式返回。接着立即使用 “解构赋值” 将该元组解构成两个变量，
            // `i` 和 `data_segment`。
            for (i, data_segment) in chunked_data.enumerate() {
                println!("data segment {} is \"{}\"", i, data_segment);

                // 用单独的线程处理每一段数据
                //
                // spawn() 返回新线程的句柄（handle），我们必须拥有句柄，
                // 才能获取线程的返回值。
                //
                // 'move || -> u32' 语法表示该闭包：
                // * 没有参数（'||'）
                // * 会获取所捕获变量的所有权（'move'）
                // * 返回无符号 32 位整数（'-> u32'）
                //
                // Rust 可以根据闭包的内容推断出 '-> u32'，所以我们可以不写它。
                //
                // 试一试：删除 'move'，看看会发生什么
                children.push(thread::spawn(move || -> u32 {
                    // 计算该段的每一位的和：
                    let result = data_segment
                        // 对该段中的字符进行迭代..
                        .chars()
                        // ..把字符转成数字..
                        .map(|c| c.to_digit(10).expect("should be a digit"))
                        // ..对返回的数字类型的迭代器求和
                        .sum();

                    // println! 会锁住标准输出，这样各线程打印的内容不会交错在一起
                    println!("processed segment {}, result={}", i, result);

                    // 不需要 “return”，因为 Rust 是一种 “表达式语言”，每个代码块中
                    // 最后求值的表达式就是代码块的值。
                    result

                }));
            }


            /*************************************************************************
             * "Reduce" 阶段
             *
             * 收集中间结果，得出最终结果
             ************************************************************************/

            // 把每个线程产生的中间结果收入一个新的向量中
            let mut intermediate_sums = vec![];
            for child in children {
                // 收集每个子线程的返回值
                let intermediate_sum = child.join().unwrap();
                intermediate_sums.push(intermediate_sum);
            }

            // 把所有中间结果加起来，得到最终结果
            //
            // 我们用 “涡轮鱼” 写法 ::<> 来为 sum() 提供类型提示。
            //
            // 试一试：不使用涡轮鱼写法，而是显式地指定 intermediate_sums 的类型
            let final_result = intermediate_sums.iter().sum::<u32>();

            println!("Final sum result: {}", final_result);
        }
    }

    pub mod c20_2 {
        use std::sync::mpsc::{Sender, Receiver};
        use std::sync::mpsc;
        use std::thread;

        static NTHREADS: i32 = 3;

        pub fn main() {
            // 通道有两个端点：`Sender<T>` 和 `Receiver<T>`，其中 `T` 是要发送
            // 的消息的类型（类型标注是可选的）
            let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

            for id in 0..NTHREADS {
                // sender 端可被复制
                let thread_tx = tx.clone();

                // 每个线程都将通过通道来发送它的 id
                thread::spawn(move || {
                    // 被创建的线程取得 `thread_tx` 的所有权
                    // 每个线程都把消息放在通道的消息队列中
                    thread_tx.send(id).unwrap();

                    // 发送是一个非阻塞（non-blocking）操作，线程将在发送完消息后
                    // 会立即继续进行
                    println!("thread {} finished", id);
                });
            }

            // 所有消息都在此处被收集
            let mut ids = Vec::with_capacity(NTHREADS as usize);
            for _ in 0..NTHREADS {
                // `recv` 方法从通道中拿到一个消息
                // 若无可用消息的话，`recv` 将阻止当前线程
                ids.push(rx.recv());
            }

            // 显示消息被发送的次序
            println!("{:?}", ids);
        }

    }

    pub mod c20_3 {
        use std::path::Path;

        pub fn main() {
            // 从 `&'static str` 创建一个 `Path`
            let path = Path::new(".");

            // `display` 方法返回一个可显示（showable）的结构体
            let display = path.display();

            // `join` 使用操作系统特定的分隔符来合并路径到一个字节容器，并返回新的路径
            let new_path = path.join("a").join("b");

            // 将路径转换成一个字符串切片
            match new_path.to_str() {
                None => panic!("new path is not a valid UTF-8 sequence"),
                Some(s) => println!("new path is {}", s),
            }
        }

    }

    pub mod c20_4_1 {
        use std::error::Error;
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        pub fn main() {
            // 创建指向所需的文件的路径
            let path = Path::new("hello.txt");
            let display = path.display();

            // 以只读方式打开路径，返回 `io::Result<File>`
            let mut file = match File::open(&path) {
                // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
                Err(why) => panic!("couldn't open {}: {}", display,
                                   why.description()),
                Ok(file) => file,
            };

            // 读取文件内容到一个字符串，返回 `io::Result<usize>`
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("couldn't read {}: {}", display,
                                   why.description()),
                Ok(_) => print!("{} contains:\n{}", display, s),
            }

            // `file` 离开作用域，并且 `hello.txt` 文件将被关闭。
        }

    }

    pub mod c20_4_2 {
        static LOREM_IPSUM: &'static str =
            "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

        use std::error::Error;
        use std::io::prelude::*;
        use std::fs::File;
        use std::path::Path;

        pub fn main() {
            let path = Path::new("out/lorem_ipsum.txt");
            let display = path.display();

            // 以只写模式打开文件，返回 `io::Result<File>`
            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}",
                                   display,
                                   why.description()),
                Ok(file) => file,
            };

            // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
            match file.write_all(LOREM_IPSUM.as_bytes()) {
                Err(why) => {
                    panic!("couldn't write to {}: {}", display,
                           why.description())
                },
                Ok(_) => println!("successfully wrote to {}", display),
            }
        }

    }

    pub mod c20_4_3 {
        use std::fs::File;
        use std::io::{self, BufRead};
        use std::path::Path;

        pub fn main() {
            // 在生成输出之前，文件主机必须存在于当前路径中
            if let Ok(lines) = read_lines("./hosts") {
                // 使用迭代器，返回一个（可选）字符串
                for line in lines {
                    if let Ok(ip) = line {
                        println!("{}", ip);
                    }
                }
            }
        }

        // 输出包裹在 Result 中以允许匹配错误，
        // 将迭代器返回给文件行的读取器（Reader）。
        fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
            where P: AsRef<Path>, {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
        }

    }

    pub mod c20_5 {
        use std::process::Command;

        pub fn main() {
            let output = Command::new("rustc")
                .arg("--version")
                .output().unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e)
            });

            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout);

                print!("rustc succeeded and stdout was:\n{}", s);
            } else {
                let s = String::from_utf8_lossy(&output.stderr);

                print!("rustc failed and stderr was:\n{}", s);
            }
        }

    }

    pub mod c20_5_1 {
        use std::error::Error;
        use std::io::prelude::*;
        use std::process::{Command, Stdio};

        static PANGRAM: &'static str =
            "the quick brown fox jumped over the lazy dog\n";

        pub fn main() {
            // 启动 `wc` 命令
            let process = match Command::new("wc")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn() {
                Err(why) => panic!("couldn't spawn wc: {}", why.description()),
                Ok(process) => process,
            };

            // 将字符串写入 `wc` 的 `stdin`。
            //
            // `stdin` 拥有 `Option<ChildStdin>` 类型，不过我们已经知道这个实例不为空值，
            // 因而可以直接 `unwrap 它。
            match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
                Err(why) => panic!("couldn't write to wc stdin: {}",
                                   why.description()),
                Ok(_) => println!("sent pangram to wc"),
            }

            // 因为 `stdin` 在上面调用后就不再存活，所以它被 `drop` 了，管道也被关闭。
            //
            // 这点非常重要，因为否则 `wc` 就不会开始处理我们刚刚发送的输入。

            // `stdout` 字段也拥有 `Option<ChildStdout>` 类型，所以必需解包。
            let mut s = String::new();
            match process.stdout.unwrap().read_to_string(&mut s) {
                Err(why) => panic!("couldn't read wc stdout: {}",
                                   why.description()),
                Ok(_) => print!("wc responded with:\n{}", s),
            }
        }

    }

    pub mod c20_5_2 {
        use std::process::Command;

        pub fn main() {
            let mut child = Command::new("sleep").arg("5").spawn().unwrap();
            let _result = child.wait().unwrap();

            println!("reached end of main");
        }

    }

    pub mod c20_6 {
        use std::fs;
        use std::fs::{File, OpenOptions};
        use std::io;
        use std::io::prelude::*;
        use std::os::unix;
        use std::path::Path;

        // `% cat path` 的简单实现
        fn cat(path: &Path) -> io::Result<String> {
            let mut f = File::open(path)?;
            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }

        // `% echo s > path` 的简单实现
        fn echo(s: &str, path: &Path) -> io::Result<()> {
            let mut f = File::create(path)?;

            f.write_all(s.as_bytes())
        }

        // `% touch path` 的简单实现（忽略已存在的文件）
        fn touch(path: &Path) -> io::Result<()> {
            match OpenOptions::new().create(true).write(true).open(path) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }

        pub fn main() {
            println!("`mkdir a`");
            // 创建一个目录，返回 `io::Result<()>`
            match fs::create_dir("a") {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(_) => {},
            }

            println!("`echo hello > a/b.txt`");
            // 前面的匹配可以用 `unwrap_or_else` 方法简化
            echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });

            println!("`mkdir -p a/c/d`");
            // 递归地创建一个目录，返回 `io::Result<()>`
            fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });

            println!("`touch a/c/e.txt`");
            touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });

            println!("`ln -s ../b.txt a/c/b.txt`");
            // 创建一个符号链接，返回 `io::Resutl<()>`
            if cfg!(target_family = "unix") {
                unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
                    println!("! {:?}", why.kind());
                });
            }

            println!("`cat a/c/b.txt`");
            match cat(&Path::new("a/c/b.txt")) {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(s) => println!("> {}", s),
            }

            println!("`ls a`");
            // 读取目录的内容，返回 `io::Result<Vec<Path>>`
            match fs::read_dir("a") {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(paths) => for path in paths {
                    println!("> {:?}", path.unwrap().path());
                },
            }

            println!("`rm a/c/e.txt`");
            // 删除一个文件，返回 `io::Result<()>`
            fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });

            println!("`rmdir a/c/d`");
            // 移除一个空目录，返回 `io::Result<()>`
            fs::remove_dir("a/c/d").unwrap_or_else(|why| {
                println!("! {:?}", why.kind());
            });
        }

    }

    pub mod c20_7 {
        use std::env;

        pub fn main() {
            let args: Vec<String> = env::args().collect();

            // 第一个参数是调用本程序的路径
            println!("My path is {}.", args[0]);

            // 其余的参数是被传递给程序的命令行参数。
            // 请这样调用程序：
            //   $ ./args arg1 arg2
            println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
        }
    }

    pub mod c20_7_1 {
        use std::env;

        fn increase(number: i32) {
            println!("{}", number + 1);
        }

        fn decrease(number: i32) {
            println!("{}", number - 1);
        }

        fn help() {
            println!("usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one.");
        }

        fn main() {
            let args: Vec<String> = env::args().collect();

            match args.len() {
                // 没有传入参数
                1 => {
                    println!("My name is 'match_args'. Try passing some arguments!");
                },
                // 一个传入参数
                2 => {
                    match args[1].parse() {
                        Ok(42) => println!("This is the answer!"),
                        _ => println!("This is not the answer."),
                    }
                },
                // 传入一条命令和一个参数
                3 => {
                    let cmd = &args[1];
                    let num = &args[2];
                    // 解析数字
                    let number: i32 = match num.parse() {
                        Ok(n) => {
                            n
                        },
                        Err(_) => {
                            println!("error: second argument not an integer");
                            help();
                            return;
                        },
                    };
                    // 解析命令
                    match &cmd[..] {
                        "increase" => increase(number),
                        "decrease" => decrease(number),
                        _ => {
                            println!("error: invalid command");
                            help();
                        },
                    }
                },
                // 所有其他情况
                _ => {
                    // 显示帮助信息
                    help();
                }
            }
        }

    }

    pub mod c20_8 {
        use std::fmt;

        // 这个 extern 代码块链接到 libm 库
        #[link(name = "m")]
        extern {
            // 这个外部函数用于计算单精度复数的平方根
            fn csqrtf(z: Complex) -> Complex;

            // 这个用来计算单精度复数的复变余弦
            fn ccosf(z: Complex) -> Complex;
        }

        // 由于调用其他语言的函数被认为是不安全的，我们通常会给它们写一层安全的封装
        fn cos(z: Complex) -> Complex {
            unsafe { ccosf(z) }
        }

        pub fn main() {
            // z = -1 + 0i
            let z = Complex { re: -1., im: 0. };

            // 调用外部语言函数是不安全操作
            let z_sqrt = unsafe { csqrtf(z) };

            println!("the square root of {:?} is {:?}", z, z_sqrt);

            // 调用不安全操作的安全的 API 封装
            println!("cos({:?}) = {:?}", z, cos(z));
        }

        // 单精度复数的最简实现
        #[repr(C)]
        #[derive(Clone, Copy)]
        struct Complex {
            re: f32,
            im: f32,
        }

        impl fmt::Debug for Complex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.im < 0. {
                    write!(f, "{}-{}i", self.re, -self.im)
                } else {
                    write!(f, "{}+{}i", self.re, self.im)
                }
            }
        }

    }
}


fn main() {
    c20::c20_8::main();
}