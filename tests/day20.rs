mod common;

fn read() -> (Algo, Image) {
    parse_input(common::read_test_input("data/day-20/input.txt")).unwrap()
}

fn read_test() -> (Algo, Image) {
    parse_input(common::read_test_input("data/day-20/input_test.txt")).unwrap()
}

fn parse_input(lines: Vec<String>) -> Option<(Algo, Image)> {
    let algo = Algo::parse(&lines[0]);
    let image: Option<Image> = Image::parse(&lines[2..]);
    algo.zip(image)
}

type AlgoDataFrame = u8;
const ALGO_FRAME_WIDTH: usize = std::mem::size_of::<AlgoDataFrame>() * 8;
const ALGO_LENGTH: usize = 512;
const ALGO_NUM_FRAMES: usize = ALGO_LENGTH / ALGO_FRAME_WIDTH;
type AlgoData = [AlgoDataFrame; ALGO_NUM_FRAMES];

#[derive(Debug, Clone, Copy)]
struct Algo {
    data: AlgoData,
}

type PixelsIn = usize;
const BASE: u32 = 2;
const MAX_PIXELS_IN: usize = (BASE.pow(9) - 1) as usize;

impl Algo {
    fn is_lit(&self, pixin: PixelsIn) -> bool {
        assert!(
            pixin <= MAX_PIXELS_IN,
            "invalid input pixels: {} > MAX_PIXELS_IN",
            pixin
        );
        let frame: usize = pixin / ALGO_FRAME_WIDTH;
        let mask: AlgoDataFrame =
            BASE.pow((7 - (pixin % ALGO_FRAME_WIDTH)) as u32) as AlgoDataFrame;
        mask == (self.data[frame] & mask)
    }

    fn of(lits: &[usize]) -> Self {
        let mut data: [AlgoDataFrame; ALGO_NUM_FRAMES] = [0; ALGO_NUM_FRAMES];
        for lit in lits {
            assert!(
                *lit <= MAX_PIXELS_IN,
                "invalid input pixels: {} > MAX_PIXELS_IN",
                lit
            );
            let frame: usize = *lit / ALGO_FRAME_WIDTH;
            let mask: AlgoDataFrame =
                BASE.pow((7 - (*lit % ALGO_FRAME_WIDTH)) as u32) as AlgoDataFrame;
            data[frame] |= mask;
        }

        Self::new(data)
    }

    fn new(data: AlgoData) -> Self {
        Self { data: data }
    }

    fn parse(line: &str) -> Option<Self> {
        assert_eq!(
            ALGO_LENGTH,
            line.len(),
            "expect algo line to be {} characters",
            ALGO_LENGTH
        );
        let as_bits = line.replace(".", "0").replace("#", "1");
        let mut data: [AlgoDataFrame; ALGO_NUM_FRAMES] = [0; ALGO_NUM_FRAMES];
        for frame_index in 0..ALGO_NUM_FRAMES {
            let str_index = frame_index * ALGO_FRAME_WIDTH;
            let subs = &as_bits[str_index..(str_index + ALGO_FRAME_WIDTH)];
            if let Some(parsed) = AlgoDataFrame::from_str_radix(subs, BASE).ok() {
                data[frame_index] = parsed;
            } else {
                return None;
            }
        }
        return Some(Self::new(data));
    }
}

impl ToString for Algo {
    fn to_string(&self) -> String {
        let mut out = "".to_owned();
        for frame in 0..ALGO_NUM_FRAMES {
            out = out + format!("{:08b}", self.data[frame]).as_str();
        }
        return out;
    }
}

/// an image can be represented in memory as a sparse square of u16s,
/// each one representing a 3x3 pixel region that does not overlap with
/// the square represented by the next u16.
type Pos = (usize, usize);
struct Image {
    res: usize,
    uni_lit: bool,
    data: Vec<Vec<u16>>,
}

impl Image {
    /// next (0,0) uses top row of zeros and left col of zeros, plus prev (0,0)
    fn next(&self, algo: &Algo) -> Self {
        let new_res = self.res + 2;
        //let new_data_size = (new_res + Self::res_padding(new_res)) / 3;
        let new_data_size = self.data.len() + 1;
        let mut new_data: Vec<Vec<u16>> = Vec::new();
        for row in 0..new_data_size {
            let mut new_data_row: Vec<u16> = Vec::new();
            for col in 0..new_data_size {
                let data_key = self.data_key(&(row, col));
                new_data_row.push(Self::next_data(&data_key, algo));
            }
            new_data.push(new_data_row);
        }
        Self {
            res: new_res,
            uni_lit: (self.uni_lit && algo.is_lit(MAX_PIXELS_IN))
                || (!self.uni_lit && algo.is_lit(0)),
            data: new_data,
        }
    }

    /// 000111
    /// 000111
    /// 00...1
    /// 22...3
    /// 22...3
    /// 222333
    ///
    ///
    /// 0 <- key[0](4,5,7,8) + key[1](3,6) + key[2](1,2) + key[3](0)
    /// 1 <- key[0](5,8) + key[1](3,4,6,7) + key[2](2) + key[3](0,1)
    /// 2 <- key[0]() + key[1](3,4,5,6,7,8) + key[2]() + key[3](0,1,2)
    /// 3 <- key[0](7,8) + key[1](6) + key[2](1,2,4,5) + key[3](0,3)
    /// 4 <- key[0](8) + key[1](6,7) + key[2](2,5) + key[3](0,1,3,4)
    /// 5 <- key[0]() + key[1](6,7,8) + key[2]() + key[3](0,1,2,3,4,5)
    /// 6 <- key[0]() + key[1]() + key[2](1,2,4,5,7,8) + key[3](0,3,6)
    /// 7 <- key[0]() + key[1]() + key[2](2,5,8) + key[3](0,1,3,4,6,7)
    /// 8 <- key[3]
    ///
    ///   04,05,13  05,13,14  13,14,15
    ///   07,08,16  08,16,17  16,17,18
    ///   21,22,30  22,30,31  30,31,32
    ///
    ///   07,08,16  08,16,17  16,17,18
    ///   21,22,30  22,30,31  30,31,32
    ///   24,25,33  25,33,34  33,34,35
    ///
    ///   21,22,30  22,30,31  30,31,32
    ///   24,25,33  25,33,34  33,34,35
    ///   27,28,36  28,36,37  36,37,38
    fn next_data(data_key: &DataKey, algo: &Algo) -> u16 {
        let mut new_value: u16 = 0;
        let algo_inputs = data_key.algo_inputs();
        for p in 0..9 {
            if algo.is_lit(algo_inputs[p]) {
                new_value |= BASE.pow((8 - p) as u32) as u16
            }
        }
        return new_value;
    }

    /// This is the key for computing the solution directly from sparse
    /// data.
    /// 
    /// Because each iteration of an image increases the defined resolution by two, one
    /// unit in each direction, and because we always align the sparse data matrix origin 
    /// to the extreme upper left of its image, the sparse data matrix of the next image 
    /// always starts one pixel up and one pixel left relative to the current image matrix.
    /// 
    /// And since every sparse data cell in the next image requires a 5x5 square of input 
    /// pixels to calculate all of its 3x3 output pixels, we can provide that input data 
    /// from just 4 sparse data values in the current image.
    /// 
    /// 0|0|0||1|1|1
    /// -+-+-++-+-+-
    /// 0|0|0||1|1|1
    /// -+-+-++-+-+-
    /// 0|0| || | |1
    /// =‡=‡=‡‡=‡=‡=
    /// 2|2| || | |3
    /// -+-+-++-+-+-
    /// 2|2| || | |3
    /// -+-+-++-+-+-
    /// 2|2|2||3|3|3
    ///
    /// The normal case for this is to provide a 4-element array of values from the 
    /// current data. 
    /// 
    ///   [ 
    ///     (row-1, col-1), (row-1, col),
    ///     (row,   col-1), (row,   col)
    ///   ]
    /// 
    /// The edge cases consist of all the values on the edge of the current image.
    /// The current image data may not have stored all the values needed for the next
    /// image, so for each cell past the edge, the correct default of zero or MAX_PIXELS_IN
    /// must be provided in its place.
    fn data_key(&self, data_pos: &Pos) -> DataKey {
        let (row, col) = *data_pos;
        let def_val = if self.uni_lit {
            MAX_PIXELS_IN as u16
        } else {
            0
        };
        let mut key: [u16; 4] = [def_val; 4];
        let data_size = self.data.len();
        if row > 0 && row - 1 < data_size && col > 0 && col - 1 < data_size {
            key[0] = self.data[row - 1][col - 1];
        }
        if row > 0 && row - 1 < data_size && col < data_size {
            key[1] = self.data[row - 1][col];
        }
        if col > 0 && col - 1 < data_size && row < data_size {
            key[2] = self.data[row][col - 1];
        }
        if row < data_size && col < data_size {
            key[3] = self.data[row][col];
        }
        DataKey::new(&key)
    }

    fn res_padding(res: usize) -> usize {
        (3 - (res % 3)) % 3
    }

    fn parse(lines: &[String]) -> Option<Self> {
        let mut inputs: Vec<String> = lines
            .iter()
            .filter(|line| !line.is_empty())
            .cloned()
            .collect();
        let res = inputs.len();
        let append_size = Self::res_padding(res);
        inputs.append(&mut (0..append_size).map(|_| Self::ndots(res)).collect());
        let suffix = Self::ndots(append_size);
        let all_inputs: Vec<String> = inputs
            .iter()
            .map(|line| line.to_owned() + suffix.as_str())
            .map(|line| line.replace(".", "0").replace("#", "1"))
            .collect();
        let full_size = res + append_size;
        let raw_size = full_size / 3;
        let mut data: Vec<Vec<u16>> = Vec::new();
        for row in 0..raw_size {
            let mut data_row: Vec<u16> = Vec::new();
            let i_row = row * 3;
            for col in 0..raw_size {
                let i_col = col * 3;
                let com: String = "".to_owned()
                    + &all_inputs[i_row][i_col..(i_col + 3)]
                    + &all_inputs[i_row + 1][i_col..(i_col + 3)]
                    + &all_inputs[i_row + 2][i_col..(i_col + 3)];
                if let Some(parsed) = u16::from_str_radix(&com, 2).ok() {
                    data_row.push(parsed);
                } else {
                    return None;
                }
            }
            data.push(data_row);
        }
        Some(Self {
            res: res,
            uni_lit: false,
            data: data,
        })
    }

    fn ndots(n: usize) -> String {
        let mut dots: String = "".to_owned();
        for _ in 0..n {
            dots = dots + ".";
        }
        dots
    }

    fn count_lit(&self) -> usize {
        let value = self.to_string();
        value.chars().filter(|c| *c == '#').fold(0, |a, _| a + 1)
    }

    fn num9(text: &str) -> usize {
        let stripped = text.chars().fold("".to_owned(), |a, v| {
            if v == '.' {
                a + "0"
            } else if v == '#' {
                a + "1"
            } else {
                a
            }
        });
        assert_eq!(9, stripped.len());
        usize::from_str_radix(&stripped, 2).ok().unwrap()
    }
}

impl ToString for Image {
    fn to_string(&self) -> String {
        let mut lines: Vec<String> = Vec::new();
        let res = self.res;
        for row in 0..self.data.len() {
            let mut s_0: String = "".to_owned();
            let mut s_1: String = "".to_owned();
            let mut s_2: String = "".to_owned();
            for datum in &self.data[row][0..] {
                let formatted = format!("{:09b}", datum);
                s_0 = s_0 + &formatted[0..3];
                s_1 = s_1 + &formatted[3..6];
                s_2 = s_2 + &formatted[6..9];
            }
            lines.push(s_0);
            lines.push(s_1);
            lines.push(s_2);
        }

        let results: Vec<String> = lines
            .iter()
            .map(|line| line[0..res].to_owned())
            .take(res)
            .collect();
        results.join("\n").replace("0", ".").replace("1", "#")
    }
}

type DataKeyArr = [u16; 4];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct DataKey {
    arr: DataKeyArr,
}

/// 0|0|0||1|1|1
/// -+-+-++-+-+-
/// 0|0|0||1|1|1
/// -+-+-++-+-+-
/// 0|0| || | |1
/// =‡=‡=‡‡=‡=‡=
/// 2|2| || | |3
/// -+-+-++-+-+-
/// 2|2| || | |3
/// -+-+-++-+-+-
/// 2|2|2||3|3|3
///
/// 0 <- key[0](4,5,7,8) + key[1](3,6) + key[2](1,2) + key[3](0)
/// 1 <- key[0](5,8) + key[1](3,4,6,7) + key[2](2) + key[3](0,1)
/// 2 <- key[0]() + key[1](3,4,5,6,7,8) + key[2]() + key[3](0,1,2)
/// 3 <- key[0](7,8) + key[1](6) + key[2](1,2,4,5) + key[3](0,3)
/// 4 <- key[0](8) + key[1](6,7) + key[2](2,5) + key[3](0,1,3,4)
/// 5 <- key[0]() + key[1](6,7,8) + key[2]() + key[3](0,1,2,3,4,5)
/// 6 <- key[0]() + key[1]() + key[2](1,2,4,5,7,8) + key[3](0,3,6)
/// 7 <- key[0]() + key[1]() + key[2](2,5,8) + key[3](0,1,3,4,6,7)
/// 8 <- key[3]
///
///   04,05,13  05,13,14  13,14,15
///   07,08,16  08,16,17  16,17,18
///   21,22,30  22,30,31  30,31,32
///
///   07,08,16  08,16,17  16,17,18
///   21,22,30  22,30,31  30,31,32
///   24,25,33  25,33,34  33,34,35
///
///   21,22,30  22,30,31  30,31,32
///   24,25,33  25,33,34  33,34,35
///   27,28,36  28,36,37  36,37,38
impl DataKey {
    fn algo_input_p0(&self) -> u16 {
        self.algo_input_by_mask(&Self::P0)
    }
    fn algo_input_p1(&self) -> u16 {
        self.algo_input_by_mask(&Self::P1)
    }
    fn algo_input_p2(&self) -> u16 {
        self.algo_input_by_mask(&Self::P2)
    }
    fn algo_input_p3(&self) -> u16 {
        self.algo_input_by_mask(&Self::P3)
    }
    fn algo_input_p4(&self) -> u16 {
        self.algo_input_by_mask(&Self::P4)
    }
    fn algo_input_p5(&self) -> u16 {
        self.algo_input_by_mask(&Self::P5)
    }
    fn algo_input_p6(&self) -> u16 {
        self.algo_input_by_mask(&Self::P6)
    }
    fn algo_input_p7(&self) -> u16 {
        self.algo_input_by_mask(&Self::P7)
    }
    fn algo_input_p8(&self) -> u16 {
        self.algo_input_by_mask(&Self::P8)
    }

    fn algo_inputs(&self) -> [usize; 9] {
        [
            self.algo_input_p0() as usize,
            self.algo_input_p1() as usize,
            self.algo_input_p2() as usize,
            self.algo_input_p3() as usize,
            self.algo_input_p4() as usize,
            self.algo_input_p5() as usize,
            self.algo_input_p6() as usize,
            self.algo_input_p7() as usize,
            self.algo_input_p8() as usize,
        ]
    }

    fn algo_input_by_mask(&self, mapping: &DataKeyMapping) -> u16 {
        assert_eq!(9, mapping.len());
        let mut p: u16 = 0;
        for imask in 0..9 {
            let (iarr, ival) = mapping[imask];
            p |= Self::to_mask(self.arr[iarr], ival, imask);
        }
        return p;
    }

    fn new(arr: &DataKeyArr) -> Self {
        Self { arr: *arr }
    }

    fn to_mask(value: u16, ival: usize, imask: usize) -> u16 {
        let valmask: u16 = BASE.pow(8 - (ival as u32)) as u16;
        let mask: u16 = value & valmask;
        if ival > imask {
            return mask << (ival - imask);
        } else if ival < imask {
            return mask >> (imask - ival);
        } else {
            return mask;
        }
    }
}

type DataKeyMapping = [(usize, usize); 9];
trait DataKeyMappings {
    const P0: DataKeyMapping;
    const P1: DataKeyMapping;
    const P2: DataKeyMapping;
    const P3: DataKeyMapping;
    const P4: DataKeyMapping;
    const P5: DataKeyMapping;
    const P6: DataKeyMapping;
    const P7: DataKeyMapping;
    const P8: DataKeyMapping;
}

impl DataKeyMappings for DataKey {
    /// 04,05,13
    /// 07,08,16
    /// 21,22,30
    const P0: DataKeyMapping = [
        (0, 4),
        (0, 5),
        (1, 3),
        (0, 7),
        (0, 8),
        (1, 6),
        (2, 1),
        (2, 2),
        (3, 0),
    ];

    /// 05,13,14
    /// 08,16,17
    /// 22,30,31
    const P1: DataKeyMapping = [
        (0, 5),
        (1, 3),
        (1, 4),
        (0, 8),
        (1, 6),
        (1, 7),
        (2, 2),
        (3, 0),
        (3, 1),
    ];

    /// 13,14,15
    /// 16,17,18
    /// 30,31,32
    const P2: DataKeyMapping = [
        (1, 3),
        (1, 4),
        (1, 5),
        (1, 6),
        (1, 7),
        (1, 8),
        (3, 0),
        (3, 1),
        (3, 2),
    ];

    /// 07,08,16
    /// 21,22,30
    /// 24,25,33
    const P3: DataKeyMapping = [
        (0, 7),
        (0, 8),
        (1, 6),
        (2, 1),
        (2, 2),
        (3, 0),
        (2, 4),
        (2, 5),
        (3, 3),
    ];

    /// 08,16,17  16,17,18
    /// 22,30,31  30,31,32
    /// 25,33,34  33,34,35
    const P4: DataKeyMapping = [
        (0, 8),
        (1, 6),
        (1, 7),
        (2, 2),
        (3, 0),
        (3, 1),
        (2, 5),
        (3, 3),
        (3, 4),
    ];

    /// 16,17,18
    /// 30,31,32
    /// 33,34,35
    const P5: DataKeyMapping = [
        (1, 6),
        (1, 7),
        (1, 8),
        (3, 0),
        (3, 1),
        (3, 2),
        (3, 3),
        (3, 4),
        (3, 5),
    ];

    ///   21,22,30  22,30,31  30,31,32
    ///   24,25,33  25,33,34  33,34,35
    ///   27,28,36  28,36,37  36,37,38
    const P6: DataKeyMapping = [
        (2, 1),
        (2, 2),
        (3, 0),
        (2, 4),
        (2, 5),
        (3, 3),
        (2, 7),
        (2, 8),
        (3, 6),
    ];

    /// 22,30,31  30,31,32
    /// 25,33,34  33,34,35
    /// 28,36,37  36,37,38
    const P7: DataKeyMapping = [
        (2, 2),
        (3, 0),
        (3, 1),
        (2, 5),
        (3, 3),
        (3, 4),
        (2, 8),
        (3, 6),
        (3, 7),
    ];

    /// 30,31,32
    /// 33,34,35
    /// 36,37,38
    const P8: DataKeyMapping = [
        (3, 0),
        (3, 1),
        (3, 2),
        (3, 3),
        (3, 4),
        (3, 5),
        (3, 6),
        (3, 7),
        (3, 8),
    ];
}


#[test]
fn day20_test_data_key() {
    let zeros = DataKey::new(&[0; 4]).algo_inputs();
    assert!(!(zeros.iter().fold(false, |a, v| a || (*v != 0))));
}

fn checker_algo() -> Algo {
    Algo::of(&[
        Image::num9("#........"),
        Image::num9("..#......"),
        Image::num9("......#.."),
        Image::num9("........#"),
        Image::num9("#.#......"),
        Image::num9("#.....#.."),
        Image::num9("..#.....#"),
        Image::num9("......#.#"),
        Image::num9("....#...."),
        Image::num9("#...#...."),
        Image::num9("..#.#...."),
        Image::num9("....#.#.."),
        Image::num9("....#...#"),
        Image::num9("#.#.#...."),
        Image::num9("#...#.#.."),
        Image::num9("..#.#...#"),
        Image::num9("....#.#.#"),
        Image::num9("#.#.#.#.#"),
    ])
}

#[test]
fn day20_test_image_next_odd() {
    let algo = checker_algo();

    let image = Image::parse(&["#".to_owned()]).unwrap();
    println!("image0:\n{}", image.to_string());
    let image1 = image.next(&algo);
    println!("image1:\n{}", image1.to_string());
    let image2 = image1.next(&algo);
    println!("image2:\n{}", image2.to_string());
    let image3 = image2.next(&algo);
    println!("image3:\n{}", image3.to_string());
    let image4 = image3.next(&algo);
    println!("image4:\n{}", image4.to_string());
}

#[test]
fn day20_test_image_next_even() {
    let algo = checker_algo();

    let image = Image::parse(&["#.".to_owned(), ".#".to_owned()]).unwrap();
    println!("image0:\n{}", image.to_string());
    let image1 = image.next(&algo);
    println!("image1:\n{}", image1.to_string());
    let image2 = image1.next(&algo);
    println!("image2:\n{}", image2.to_string());
    let image3 = image2.next(&algo);
    println!("image3:\n{}", image3.to_string());
    let image4 = image3.next(&algo);
    println!("image4:\n{}", image4.to_string());
}

#[test]
fn day20_test_parse_algo() {
    let (algo, _) = read_test();
    assert_eq!(
        false,
        algo.is_lit(0),
        "expect 0 is not lit: {}",
        algo.to_string()
    );
    assert_eq!(
        true,
        algo.is_lit(10),
        "expect 10 is lit: {}",
        algo.to_string()
    );
    assert_eq!(
        true,
        algo.is_lit(20),
        "expect 20 is lit: {}",
        algo.to_string()
    );
    assert_eq!(
        false,
        algo.is_lit(22),
        "expect 22 is not lit: {}",
        algo.to_string()
    );
    assert_eq!(
        true,
        algo.is_lit(34),
        "expect 34 is lit: {}",
        algo.to_string()
    );
    assert_eq!(
        true,
        algo.is_lit(511),
        "expect 511 is lit: {}",
        algo.to_string()
    );
    assert_eq!(
        true,
        algo.is_lit(512),
        "expect 512 is lit: {}",
        algo.to_string()
    );
}

#[test]
fn day20_test_parse_image() {
    let (_, image) = read_test();

    assert_eq!(
        "#..#.
#....
##..#
..#..
..###"
            .to_owned(),
        image.to_string(),
        "expect same output"
    );
}

#[test]
fn day20pre_part1() {
    let (algo, image) = read_test();
    println!("image0:\n\n{}\n", image.to_string());
    let image1 = image.next(&algo);
    assert_eq!(
        ".##.##.
#..#.#.
##.#..#
####..#
.#..##.
..##..#
...#.#.",
        image1.to_string()
    );
    assert_eq!(24, image1.count_lit());
    let image2 = image1.next(&algo);
    assert_eq!(
        ".......#.
.#..#.#..
#.#...###
#...##.#.
#.....#.#
.#.#####.
..#.#####
...##.##.
....###..",
        image2.to_string()
    );
    assert_eq!(35, image2.count_lit());
}

#[test]
fn day20part1() {
    let (algo, image) = read();
    println!("image0:\n\n{}\n", image.to_string());
    let image1 = image.next(&algo);
    println!("image1:\n\n{}\n", image1.to_string());
    let image2 = image1.next(&algo);
    println!("image2:\n\n{}\n", image2.to_string());
    assert_ne!(5698, image2.count_lit(), "too high");
    assert_ne!(5353, image2.count_lit(), "too low");
    assert_ne!(5707, image2.count_lit());
    // omg, how did it occur to me to look at cases of 0 and 511?
    // anyway, that's the trick.
    assert_eq!(5563, image2.count_lit());
}

#[test]
fn day20pre_part2() {
    let (algo, image) = read_test();
    let final_image = (0..50).fold(image, |a, _| a.next(&algo));
    assert_eq!(3351, final_image.count_lit());
}

#[test]
fn day20part2() {
    let (algo, image) = read();
    let final_image = (0..50).fold(image, |a, _| a.next(&algo));
    assert_eq!(19743, final_image.count_lit());
}
