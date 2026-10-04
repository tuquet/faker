use rand::Rng;
use serde_json::{json, Value};
use uuid::Uuid;

pub fn to_ascii_slug(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        let r = match c {
            'a' | 'A' | 'à' | 'À' | 'á' | 'Á' | 'ả' | 'Ả' | 'ã' | 'Ã' | 'ạ' | 'Ạ'
            | 'ă' | 'Ă' | 'ằ' | 'Ằ' | 'ắ' | 'Ắ' | 'ẳ' | 'Ẳ' | 'ẵ' | 'Ẵ' | 'ặ' | 'Ặ'
            | 'â' | 'Â' | 'ầ' | 'Ầ' | 'ấ' | 'Ấ' | 'ẩ' | 'Ẩ' | 'ẫ' | 'Ẫ' | 'ậ' | 'Ậ' => 'a',
            'd' | 'D' | 'đ' | 'Đ' => 'd',
            'e' | 'E' | 'è' | 'È' | 'é' | 'É' | 'ẻ' | 'Ẻ' | 'ẽ' | 'Ẽ' | 'ẹ' | 'Ẹ'
            | 'ê' | 'Ê' | 'ề' | 'Ề' | 'ế' | 'Ế' | 'ể' | 'Ể' | 'ễ' | 'Ễ' | 'ệ' | 'Ệ' => 'e',
            'i' | 'I' | 'ì' | 'Ì' | 'í' | 'Í' | 'ỉ' | 'Ỉ' | 'ĩ' | 'Ĩ' | 'ị' | 'Ị' => 'i',
            'o' | 'O' | 'ò' | 'Ò' | 'ó' | 'Ó' | 'ỏ' | 'Ỏ' | 'õ' | 'Õ' | 'ọ' | 'Ọ'
            | 'ô' | 'Ô' | 'ồ' | 'Ồ' | 'ố' | 'Ố' | 'ổ' | 'Ổ' | 'ỗ' | 'Ỗ' | 'ộ' | 'Ộ'
            | 'ơ' | 'Ơ' | 'ờ' | 'Ờ' | 'ớ' | 'Ớ' | 'ở' | 'Ở' | 'ỡ' | 'Ỡ' | 'ợ' | 'Ợ' => 'o',
            'u' | 'U' | 'ù' | 'Ù' | 'ú' | 'Ú' | 'ủ' | 'Ủ' | 'ũ' | 'Ũ' | 'ụ' | 'Ụ'
            | 'ư' | 'Ư' | 'ừ' | 'Ừ' | 'ứ' | 'Ứ' | 'ử' | 'Ử' | 'ữ' | 'Ữ' | 'ự' | 'Ự' => 'u',
            'y' | 'Y' | 'ỳ' | 'Ỳ' | 'ý' | 'Ý' | 'ỷ' | 'Ỷ' | 'ỹ' | 'Ỹ' | 'ỵ' | 'Ỵ' => 'y',
            other => {
                if other.is_ascii_alphanumeric() {
                    other.to_ascii_lowercase()
                } else {
                    '_'
                }
            }
        };
        if r != '_' || (!out.is_empty() && !out.ends_with('_')) {
            out.push(r);
        }
    }
    out.trim_matches('_').to_string()
}

struct DistrictLocation {
    name: &'static str,
    wards: &'static [&'static str],
    streets: &'static [&'static str],
}

struct ProvinceLocation {
    code: &'static str,
    city: &'static str,
    zipcode: &'static str,
    districts: &'static [DistrictLocation],
}

static PROVINCES_VN: &[ProvinceLocation] = &[
    ProvinceLocation {
        code: "001",
        city: "Hà Nội",
        zipcode: "100000",
        districts: &[
            DistrictLocation {
                name: "Quận Hoàn Kiếm",
                wards: &["Phường Tràng Tiền", "Phường Hàng Bạc", "Phường Hàng Bài", "Phường Lý Thái Tổ"],
                streets: &["Phố Tràng Tiền", "Phố Đinh Tiên Hoàng", "Phố Hàng Khay", "Phố Bà Triệu", "Phố Lý Thường Kiệt"],
            },
            DistrictLocation {
                name: "Quận Ba Đình",
                wards: &["Phường Điện Biên", "Phường Kim Mã", "Phường Liễu Giai", "Phường Đội Cấn"],
                streets: &["Đường Kim Mã", "Đường Liễu Giai", "Đường Đội Cấn", "Đường Quán Thánh", "Đường Phan Đình Phùng"],
            },
            DistrictLocation {
                name: "Quận Cầu Giấy",
                wards: &["Phường Dịch Vọng Hậu", "Phường Nghĩa Tân", "Phường Mai Dịch", "Phường Yên Hòa"],
                streets: &["Đường Duy Tân", "Đường Xuân Thủy", "Đường Cầu Giấy", "Đường Trần Thái Tông", "Đường Hoàng Quốc Việt"],
            },
            DistrictLocation {
                name: "Quận Đống Đa",
                wards: &["Phường Láng Hạ", "Phường Ô Chợ Dừa", "Phường Văn Miếu", "Phường Khâm Thiên"],
                streets: &["Đường Thái Hà", "Đường Huỳnh Thúc Kháng", "Đường Xã Đàn", "Đường Chùa Bộc", "Đường Tôn Đức Thắng"],
            },
            DistrictLocation {
                name: "Quận Hai Bà Trưng",
                wards: &["Phường Bạch Mai", "Phường Bách Khoa", "Phường Minh Khai", "Phường Lê Đại Hành"],
                streets: &["Đường Phố Huế", "Đường Đại Cồ Việt", "Đường Bạch Mai", "Đường Minh Khai", "Đường Trần Khát Chân"],
            },
        ],
    },
    ProvinceLocation {
        code: "079",
        city: "TP Hồ Chí Minh",
        zipcode: "700000",
        districts: &[
            DistrictLocation {
                name: "Quận 1",
                wards: &["Phường Bến Nghé", "Phường Bến Thành", "Phường Đa Kao", "Phường Tân Định"],
                streets: &["Đường Nguyễn Huệ", "Đường Lê Lợi", "Đường Đồng Khởi", "Đường Pasteur", "Đường Nam Kỳ Khởi Nghĩa", "Đường Hai Bà Trưng"],
            },
            DistrictLocation {
                name: "Quận 3",
                wards: &["Phường Võ Thị Sáu", "Phường 1", "Phường 4", "Phường 9"],
                streets: &["Đường Nguyễn Thị Minh Khai", "Đường Điện Biên Phủ", "Đường Cách Mạng Tháng 8", "Đường Lý Chính Thắng", "Đường Trương Định"],
            },
            DistrictLocation {
                name: "Quận Bình Thạnh",
                wards: &["Phường 25", "Phường 26", "Phường 19", "Phường 15"],
                streets: &["Đường Điện Biên Phủ", "Đường Xô Viết Nghệ Tĩnh", "Đường Bạch Đằng", "Đường Đinh Bộ Lĩnh", "Đường Nơ Trang Long"],
            },
            DistrictLocation {
                name: "Quận Tân Bình",
                wards: &["Phường 2", "Phường 12", "Phường 15", "Phường 4"],
                streets: &["Đường Cộng Hòa", "Đường Trường Chinh", "Đường Hoàng Văn Thụ", "Đường Phổ Quang", "Đường Lê Văn Sỹ"],
            },
            DistrictLocation {
                name: "TP Thủ Đức",
                wards: &["Phường Thảo Điền", "Phường An Phú", "Phường Linh Trung", "Phường Hiệp Phú"],
                streets: &["Đường Võ Văn Ngân", "Đường Đỗ Xuân Hợp", "Đại lộ Mai Chí Thọ", "Đường Song Hành", "Đường Nguyễn Duy Trinh"],
            },
        ],
    },
    ProvinceLocation {
        code: "048",
        city: "Đà Nẵng",
        zipcode: "550000",
        districts: &[
            DistrictLocation {
                name: "Quận Hải Châu",
                wards: &["Phường Hải Châu 1", "Phường Thạch Thang", "Phường Hòa Cường Bắc"],
                streets: &["Đường Bạch Đằng", "Đường Trần Phú", "Đường Lê Duẩn", "Đường Nguyễn Văn Linh", "Đường Hùng Vương"],
            },
            DistrictLocation {
                name: "Quận Sơn Trà",
                wards: &["Phường An Hải Bắc", "Phường Phước Mỹ", "Phường Nại Hiên Đông"],
                streets: &["Đường Võ Nguyên Giáp", "Đường Phạm Văn Đồng", "Đường Ngô Quyền", "Đường Hoàng Sa"],
            },
            DistrictLocation {
                name: "Quận Thanh Khê",
                wards: &["Phường Tam Thuận", "Phường Xuân Hà", "Phường Vĩnh Trung"],
                streets: &["Đường Điện Biên Phủ", "Đường Hà Huy Tập", "Đường Lê Duẩn", "Đường Hùng Vương"],
            },
        ],
    },
    ProvinceLocation {
        code: "031",
        city: "Hải Phòng",
        zipcode: "180000",
        districts: &[
            DistrictLocation {
                name: "Quận Hồng Bàng",
                wards: &["Phường Hoàng Văn Thụ", "Phường Minh Khai", "Phường Phan Bội Châu"],
                streets: &["Đường Đinh Tiên Hoàng", "Đường Hoàng Văn Thụ", "Đường Quang Trung", "Đường Trần Phú"],
            },
            DistrictLocation {
                name: "Quận Ngô Quyền",
                wards: &["Phường Lạc Viên", "Phường Cầu Đất", "Phường Lương Khánh Thiện"],
                streets: &["Đường Lê Lợi", "Đường Cầu Đất", "Đường Đà Nẵng", "Đường Lạch Tray"],
            },
            DistrictLocation {
                name: "Quận Lê Chân",
                wards: &["Phường An Biên", "Phường Cát Dài", "Phường Hàng Kênh"],
                streets: &["Đường Tô Hiệu", "Đường Mê Linh", "Đường Trần Nguyên Hãn"],
            },
        ],
    },
    ProvinceLocation {
        code: "092",
        city: "Cần Thơ",
        zipcode: "900000",
        districts: &[
            DistrictLocation {
                name: "Quận Ninh Kiều",
                wards: &["Phường Tân An", "Phường An Cư", "Phường An Phú", "Phường Xuân Khánh"],
                streets: &["Đường 30 Tháng 4", "Đại lộ Hòa Bình", "Đường Nguyễn Trãi", "Đường Mậu Thân"],
            },
            DistrictLocation {
                name: "Quận Cái Răng",
                wards: &["Phường Lê Bình", "Phường Hưng Phú", "Phường Ba Láng"],
                streets: &["Đường Quang Trung", "Đường Phạm Hùng", "Đường Võ Nguyên Giáp"],
            },
        ],
    },
    ProvinceLocation {
        code: "074",
        city: "Bình Dương",
        zipcode: "820000",
        districts: &[
            DistrictLocation {
                name: "TP Thủ Dầu Một",
                wards: &["Phường Phú Hòa", "Phường Phú Lợi", "Phường Hiệp Thành", "Phường Chánh Nghĩa"],
                streets: &["Đại lộ Bình Dương", "Đường Yersin", "Đường Cách Mạng Tháng 8", "Đường Phú Lợi"],
            },
            DistrictLocation {
                name: "TP Thuận An",
                wards: &["Phường Lái Thiêu", "Phường An Phú", "Phường Bình Hòa"],
                streets: &["Đường DT743", "Đường Nguyễn Trãi", "Đường Cách Mạng Tháng 8"],
            },
        ],
    },
    ProvinceLocation {
        code: "075",
        city: "Đồng Nai",
        zipcode: "810000",
        districts: &[
            DistrictLocation {
                name: "TP Biên Hòa",
                wards: &["Phường Tân Phong", "Phường Trảng Dài", "Phường Thống Nhất", "Phường Quyết Thắng"],
                streets: &["Đường Đồng Khởi", "Đường Phạm Văn Thuận", "Đường Nguyễn Ái Quốc", "Đường Võ Thị Sáu"],
            },
        ],
    },
    ProvinceLocation {
        code: "056",
        city: "Khánh Hòa",
        zipcode: "650000",
        districts: &[
            DistrictLocation {
                name: "TP Nha Trang",
                wards: &["Phường Lộc Thọ", "Phường Tân Lập", "Phường Phương Sài", "Phường Vĩnh Hải"],
                streets: &["Đường Trần Phú", "Đường 2 Tháng 4", "Đường Lê Thánh Tôn", "Đường Thái Nguyên"],
            },
        ],
    },
    ProvinceLocation {
        code: "022",
        city: "Quảng Ninh",
        zipcode: "200000",
        districts: &[
            DistrictLocation {
                name: "TP Hạ Long",
                wards: &["Phường Bãi Cháy", "Phường Hồng Gai", "Phường Cao Xanh", "Phường Bạch Đằng"],
                streets: &["Đường Hạ Long", "Đường Lê Thánh Tông", "Đường Trần Hưng Đạo"],
            },
        ],
    },
    ProvinceLocation {
        code: "046",
        city: "Thừa Thiên Huế",
        zipcode: "530000",
        districts: &[
            DistrictLocation {
                name: "TP Huế",
                wards: &["Phường Vĩnh Ninh", "Phường Phú Nhuận", "Phường Phú Hội", "Phường Thuận Hòa"],
                streets: &["Đường Lê Lợi", "Đường Hùng Vương", "Đường Nguyễn Huệ", "Đường Đống Đa"],
            },
        ],
    },
];

static WEIGHTED_SURNAMES_VN: &[(&str, u32)] = &[
    ("Nguyễn", 384),
    ("Trần", 110),
    ("Lê", 95),
    ("Phạm", 71),
    ("Hoàng", 30),
    ("Huỳnh", 21),
    ("Phan", 45),
    ("Vũ", 20),
    ("Võ", 19),
    ("Đặng", 21),
    ("Bùi", 20),
    ("Đỗ", 14),
    ("Hồ", 13),
    ("Ngô", 13),
    ("Dương", 10),
    ("Lý", 7),
    ("Đinh", 8),
    ("Đoàn", 7),
    ("Lâm", 7),
    ("Mai", 6),
    ("Trịnh", 6),
    ("Đào", 5),
    ("Cao", 5),
    ("Hà", 5),
    ("Lương", 5),
    ("Thái", 4),
    ("Châu", 4),
    ("Tạ", 3),
    ("Phùng", 3),
];

fn pick_weighted<'a>(rng: &mut impl rand::Rng, items: &'a [(&'a str, u32)]) -> &'a str {
    let total: u32 = items.iter().map(|(_, w)| *w).sum();
    let mut choice = rng.gen_range(0..total);
    for &(item, weight) in items {
        if choice < weight {
            return item;
        }
        choice -= weight;
    }
    items[0].0
}

fn generate_vietnamese_cccd(
    rng: &mut impl rand::Rng,
    province_code: &str,
    birth_year: u32,
    is_male: bool,
) -> String {
    let century_gender_digit = match (birth_year, is_male) {
        (1900..=1999, true) => '0',
        (1900..=1999, false) => '1',
        (2000..=2099, true) => '2',
        (2000..=2099, false) => '3',
        (2100..=2199, true) => '4',
        (2100..=2199, false) => '5',
        _ => '0',
    };
    let year_suffix = format!("{:02}", birth_year % 100);
    let random_seq = format!("{:06}", rng.gen_range(100000..=999999));
    format!("{}{}{}{}", province_code, century_gender_digit, year_suffix, random_seq)
}

fn generate_mmo_password(rng: &mut impl rand::Rng) -> String {
    let prefixes = [
        "Tuquet", "VnPro", "Shield", "Secure", "Titan", "Falcon", "Nova", "Prime", "Turbo", "Apex", "Matrix", "Cyber"
    ];
    let specials = ['@', '#', '$', '!', '&', '*'];
    let suffixes = ["acc", "pro", "mmo", "hub", "net", "top", "run", "key"];

    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let s1 = specials[rng.gen_range(0..specials.len())];
    let s2 = specials[rng.gen_range(0..specials.len())];
    let num: u32 = rng.gen_range(1000..9999);
    let suffix = suffixes[rng.gen_range(0..suffixes.len())];

    format!("{}{}{}{}{}", prefix, s1, num, s2, suffix)
}

struct UsStateLocation {
    state: &'static str,
    city: &'static str,
    zipcode: &'static str,
    streets: &'static [&'static str],
}

static US_LOCATIONS: &[UsStateLocation] = &[
    UsStateLocation {
        state: "California",
        city: "Los Angeles",
        zipcode: "90001",
        streets: &["Sunset Boulevard", "Hollywood Boulevard", "Wilshire Boulevard", "Rodeo Drive"],
    },
    UsStateLocation {
        state: "New York",
        city: "New York City",
        zipcode: "10001",
        streets: &["Broadway", "Fifth Avenue", "Wall Street", "Madison Avenue", "Park Avenue"],
    },
    UsStateLocation {
        state: "Texas",
        city: "Houston",
        zipcode: "77001",
        streets: &["Main Street", "Texas Avenue", "Post Oak Boulevard", "Westheimer Road"],
    },
    UsStateLocation {
        state: "Florida",
        city: "Miami",
        zipcode: "33101",
        streets: &["Ocean Drive", "Biscayne Boulevard", "Collins Avenue", "Brickell Avenue"],
    },
    UsStateLocation {
        state: "Washington",
        city: "Seattle",
        zipcode: "98101",
        streets: &["Pike Street", "Pine Street", "Second Avenue", "University Street"],
    },
    UsStateLocation {
        state: "Illinois",
        city: "Chicago",
        zipcode: "60601",
        streets: &["Michigan Avenue", "State Street", "Wacker Drive", "Lake Shore Drive"],
    },
];

pub fn generate_local_users(
    count: u32,
    gender_filter: Option<&str>,
    nat_filter: Option<&str>,
    avatar_style: Option<&str>,
) -> Value {
    let mut rng = rand::thread_rng();
    let nat = nat_filter.unwrap_or("VN").to_uppercase();
    let is_vn = nat.contains("VN") || nat == "ALL";
    let is_svg = avatar_style == Some("svg");

    let middle_names_male_vn = [
        "Văn", "Hữu", "Đức", "Quang", "Minh", "Thanh", "Đình", "Ngọc", "Tuấn", "Hoàng",
        "Bảo", "Gia", "Thành", "Xuân", "Trọng", "Công", "Duy", "Anh", "Quốc", "Hải"
    ];

    let first_names_male_vn = [
        "Minh", "Hoàng", "Duy", "Tuấn", "Nam", "Quân", "Long", "Đức", "Anh", "Hùng",
        "Bảo", "Huy", "Thắng", "Phong", "Khoa", "Kiên", "Dũng", "Khánh", "Trí", "Phúc",
        "Thịnh", "Bình", "Cường", "Tùng", "Sơn", "Lâm", "Hiếu", "Vinh", "Khôi", "Nhật"
    ];

    let middle_names_female_vn = [
        "Thị", "Thu", "Phương", "Mai", "Ngọc", "Thanh", "Hồng", "Khánh", "Kim", "Bảo",
        "Ánh", "Quỳnh", "Thùy", "Mỹ", "Diệu", "Tuyết", "Hương", "Trúc", "Hoàng", "Như"
    ];

    let first_names_female_vn = [
        "Linh", "Trang", "Hương", "Mai", "Lan", "Ngọc", "Hà", "Phương", "Thu", "Thảo",
        "Huyền", "Yến", "Anh", "Nhi", "Vy", "Hân", "Châu", "Trâm", "Ngân", "Dung",
        "Uyên", "Chi", "Quỳnh", "Mi", "Thư", "Tâm", "Loan", "Bích", "Ly", "Vân"
    ];

    let jobs_vn = [
        "Kỹ sư Phần mềm (Senior)", "Lập trình viên Frontend", "Lập trình viên Backend",
        "Quản lý Dự án (PM)", "Chuyên viên Phân tích Nghiệp vụ (BA)", "Kỹ sư DevOps / Cloud",
        "Trưởng nhóm Kiểm thử (QA Lead)", "Thiết kế Giao diện (UI/UX Designer)", "Chuyên viên Dữ liệu (Data Analyst)",
        "Kế toán trưởng", "Chuyên viên Nhân sự (HR)", "Trưởng phòng Kinh doanh",
        "Kiến trúc sư Giải pháp (Solution Architect)", "Giám đốc Vận hành (COO)"
    ];

    // International fallback arrays
    let first_names_male_en = [
        "James", "John", "Robert", "Michael", "William", "David", "Richard", "Joseph",
        "Thomas", "Charles", "Daniel", "Matthew", "Anthony", "Mark", "Donald", "Steven"
    ];

    let first_names_female_en = [
        "Mary", "Patricia", "Jennifer", "Linda", "Elizabeth", "Barbara", "Susan", "Jessica",
        "Sarah", "Karen", "Lisa", "Nancy", "Betty", "Margaret", "Sandra", "Ashley"
    ];

    let last_names_en = [
        "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis",
        "Rodriguez", "Martinez", "Hernandez", "Lopez", "Gonzalez", "Wilson", "Anderson", "Thomas"
    ];

    let jobs_en = [
        "Staff Software Engineer", "Product Manager", "Lead Data Scientist",
        "Cloud Solutions Architect", "UI/UX Principal", "Engineering Director"
    ];

    let telco_prefixes_vn = [
        // Viettel
        "098", "097", "096", "086", "032", "033", "034", "035", "036", "037", "038", "039",
        // Vinaphone
        "091", "094", "088", "083", "084", "085", "081", "082",
        // Mobifone
        "090", "093", "089", "070", "079", "077", "076", "078",
    ];

    let mut results = Vec::new();

    for _ in 0..count {
        let is_male = match gender_filter {
            Some("male") => true,
            Some("female") => false,
            _ => rng.gen_bool(0.5),
        };

        let gender_str = if is_male { "male" } else { "female" };
        let title_str = if is_male { "Mr" } else { if rng.gen_bool(0.6) { "Ms" } else { "Mrs" } };

        // Realistic MMO working & registration age bracket: 18 to 45
        let age = rng.gen_range(18..=45);
        let birth_year = 2026 - age;
        let birth_month = rng.gen_range(1..=12);
        let birth_day = rng.gen_range(1..=28);
        let dob_str = format!("{:04}-{:02}-{:02}T08:00:00.000Z", birth_year, birth_month, birth_day);

        let (full_first_name, last_name, job, street_number, street_name, ward_name, district_name, city_name, state_name, country_name, postcode, id_name, id_value, phone) = if is_vn {
            let last = pick_weighted(&mut rng, WEIGHTED_SURNAMES_VN);
            let (middle, first) = if is_male {
                (
                    middle_names_male_vn[rng.gen_range(0..middle_names_male_vn.len())],
                    first_names_male_vn[rng.gen_range(0..first_names_male_vn.len())],
                )
            } else {
                (
                    middle_names_female_vn[rng.gen_range(0..middle_names_female_vn.len())],
                    first_names_female_vn[rng.gen_range(0..first_names_female_vn.len())],
                )
            };
            let first_combined = format!("{} {}", middle, first);
            let job = jobs_vn[rng.gen_range(0..jobs_vn.len())];

            // Hierarchical location matching Province -> District -> Ward -> Street
            let prov = &PROVINCES_VN[rng.gen_range(0..PROVINCES_VN.len())];
            let dist = &prov.districts[rng.gen_range(0..prov.districts.len())];
            let ward = dist.wards[rng.gen_range(0..dist.wards.len())];
            let street = dist.streets[rng.gen_range(0..dist.streets.len())];
            let st_num = rng.gen_range(1..999);

            // Ministry of Public Security compliant 12-digit CCCD
            let cccd = generate_vietnamese_cccd(&mut rng, prov.code, birth_year, is_male);

            // Valid 10-digit telco phone
            let p_prefix = telco_prefixes_vn[rng.gen_range(0..telco_prefixes_vn.len())];
            let p_num = format!("{}{:07}", p_prefix, rng.gen_range(1000000..9999999));

            (
                first_combined,
                last,
                job,
                st_num,
                street.to_string(),
                ward.to_string(),
                dist.name.to_string(),
                prov.city.to_string(),
                dist.name.to_string(),
                "Vietnam".to_string(),
                prov.zipcode.to_string(),
                "CCCD",
                cccd,
                p_num,
            )
        } else {
            let last = last_names_en[rng.gen_range(0..last_names_en.len())];
            let first = if is_male {
                first_names_male_en[rng.gen_range(0..first_names_male_en.len())]
            } else {
                first_names_female_en[rng.gen_range(0..first_names_female_en.len())]
            };
            let job = jobs_en[rng.gen_range(0..jobs_en.len())];

            let us_loc = &US_LOCATIONS[rng.gen_range(0..US_LOCATIONS.len())];
            let street = us_loc.streets[rng.gen_range(0..us_loc.streets.len())];
            let st_num = rng.gen_range(10..999);

            let ssn = format!("{:03}-{:02}-{:04}", rng.gen_range(100..900), rng.gen_range(10..99), rng.gen_range(1000..9999));
            let p_num = format!("+1 ({:03}) {:03}-{:04}", rng.gen_range(201..999), rng.gen_range(200..999), rng.gen_range(1000..9999));

            (
                first.to_string(),
                last,
                job,
                st_num,
                street.to_string(),
                "".to_string(),
                us_loc.city.to_string(),
                us_loc.city.to_string(),
                us_loc.state.to_string(),
                "United States".to_string(),
                us_loc.zipcode.to_string(),
                "SSN",
                ssn,
                p_num,
            )
        };

        // Strict RFC 5322 ASCII Email & Username sanitization
        let clean_first = to_ascii_slug(&full_first_name).replace('_', "");
        let clean_last = to_ascii_slug(last_name).replace('_', "");
        let random_suffix = rng.gen_range(10..999);
        let username = format!("{}_{}{}", clean_last, clean_first, random_suffix);
        let email = format!("{}.{}{}@tuquet.io", clean_first, clean_last, random_suffix);

        // Strong MMO Password (Upper, Lower, Number, Special - passes all strict reg rules)
        let password = generate_mmo_password(&mut rng);

        let user_uuid = Uuid::new_v4().to_string();

        // Avatar Generation: Real Human Portrait (RandomUser.me official dataset 0-99) vs Procedural Vector SVG (DiceBear)
        let (avatar_large, avatar_medium, avatar_thumb) = if is_svg {
            let avatar_seed = format!("{}-{}", username, user_uuid);
            let url = format!(
                "https://api.dicebear.com/7.x/avataaars/svg?seed={}&backgroundColor=b6e3f4,c0aede,d1d4f9,ffd5dc,ffdfbf",
                avatar_seed
            );
            (url.clone(), url.clone(), url)
        } else {
            // Authentic RandomUser Portraits (men 0-99, women 0-99, correctly partitioned by gender)
            let photo_id = rng.gen_range(0..100);
            let gender_dir = if is_male { "men" } else { "women" };
            (
                format!("https://randomuser.me/api/portraits/{}/{}.jpg", gender_dir, photo_id),
                format!("https://randomuser.me/api/portraits/med/{}/{}.jpg", gender_dir, photo_id),
                format!("https://randomuser.me/api/portraits/thumb/{}/{}.jpg", gender_dir, photo_id),
            )
        };

        // Embedded Inline SVG Data URI (Works 100% offline, zero network requests, forever permanent)
        let initials = format!(
            "{}{}",
            last_name.chars().next().unwrap_or('T'),
            full_first_name.split_whitespace().last().and_then(|s| s.chars().next()).unwrap_or('U')
        );
        let bg_color = match rng.gen_range(0..6) {
            0 => "#0284c7", // Sky
            1 => "#4f46e5", // Indigo
            2 => "#059669", // Emerald
            3 => "#d97706", // Amber
            4 => "#7c3aed", // Violet
            _ => "#e11d48", // Rose
        };
        let inline_svg = format!(
            "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 120 120'><rect width='120' height='120' rx='28' fill='{}'/><text x='60' y='72' font-family='Arial, sans-serif' font-size='42' font-weight='bold' fill='#ffffff' text-anchor='middle' dominant-baseline='middle'>{}</text></svg>",
            bg_color, initials
        );
        let inline_svg_data_uri = format!("data:image/svg+xml;utf8,{}", inline_svg);

        let user = json!({
            "gender": gender_str,
            "name": {
                "title": title_str,
                "first": full_first_name,
                "last": last_name
            },
            "job": job,
            "location": {
                "street": {
                    "number": street_number,
                    "name": street_name
                },
                "ward": ward_name,
                "district": district_name,
                "city": city_name,
                "state": state_name,
                "country": country_name,
                "postcode": postcode,
                "coordinates": {
                    "latitude": format!("{:.4}", rng.gen_range(10.0..21.0)),
                    "longitude": format!("{:.4}", rng.gen_range(105.0..108.0))
                },
                "timezone": {
                    "offset": if is_vn { "+07:00" } else { "-05:00" },
                    "description": if is_vn { "Bangkok, Hanoi, Jakarta" } else { "Eastern Time (US & Canada)" }
                }
            },
            "email": email,
            "login": {
                "uuid": user_uuid,
                "username": username,
                "password": password,
                "salt": format!("{:08x}", rng.gen::<u32>()),
                "md5": format!("{:032x}", rng.gen::<u128>()),
                "sha1": format!("{:040x}", rng.gen::<u128>()),
                "sha256": format!("{:064x}", rng.gen::<u128>())
            },
            "dob": {
                "date": dob_str,
                "age": age
            },
            "registered": {
                "date": "2023-01-15T00:00:00.000Z",
                "age": 3
            },
            "phone": phone.clone(),
            "cell": phone,
            "id": {
                "name": id_name,
                "value": id_value
            },
            "picture": {
                "large": avatar_large,
                "medium": avatar_medium,
                "thumbnail": avatar_thumb,
                "data_uri": inline_svg_data_uri
            },
            "nat": if is_vn { "VN" } else { "US" }
        });

        results.push(user);
    }

    json!({
        "results": results,
        "info": {
            "seed": "tuquet-rust-deterministic-suite",
            "results": count,
            "page": 1,
            "version": "2.0.0"
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_default_count() {
        let data = generate_local_users(10, None, None, None);
        let list = data["results"].as_array().unwrap();
        assert_eq!(list.len(), 10);
        for u in list {
            let email = u["email"].as_str().unwrap();
            assert!(email.is_ascii(), "Email must be strictly ASCII: {}", email);
            assert!(!email.contains(' '), "Email must not have spaces: {}", email);
            let img = u["picture"]["large"].as_str().unwrap();
            assert!(img.contains("portraits"));
        }
    }

    #[test]
    fn test_generate_female_us() {
        let data = generate_local_users(5, Some("female"), Some("US"), Some("svg"));
        let list = data["results"].as_array().unwrap();
        assert_eq!(list.len(), 5);
        for u in list {
            assert_eq!(u["gender"], "female");
            assert_eq!(u["nat"], "US");
            assert!(u["email"].as_str().unwrap().is_ascii());
            assert!(u["picture"]["large"].as_str().unwrap().contains("dicebear.com"));
            assert!(u["picture"]["data_uri"].as_str().unwrap().starts_with("data:image/svg+xml"));
        }
    }

    #[test]
    fn test_generate_vietnamese_users_mmo_compliance() {
        let data = generate_local_users(20, Some("male"), Some("VN"), Some("real"));
        let list = data["results"].as_array().unwrap();
        assert_eq!(list.len(), 20);
        for u in list {
            assert_eq!(u["gender"], "male");
            assert_eq!(u["nat"], "VN");
            assert!(u["job"].is_string());
            assert_eq!(u["id"]["name"], "CCCD");

            let cccd = u["id"]["value"].as_str().unwrap();
            assert_eq!(cccd.len(), 12, "CCCD must be exactly 12 digits");
            assert!(cccd.chars().all(|c| c.is_ascii_digit()), "CCCD must be numeric");

            // Verify century and gender digit in CCCD
            let age = u["dob"]["age"].as_u64().unwrap() as u32;
            let birth_year = 2026 - age;
            let expected_gender_digit = if birth_year < 2000 { '0' } else { '2' };
            assert_eq!(cccd.chars().nth(3).unwrap(), expected_gender_digit);

            // Verify 2-digit birth year matches CCCD
            let expected_year_suffix = format!("{:02}", birth_year % 100);
            assert_eq!(&cccd[4..6], &expected_year_suffix);

            // Verify Email is strictly ASCII
            let email = u["email"].as_str().unwrap();
            assert!(email.is_ascii(), "Vietnamese email must be strictly ASCII: {}", email);

            // Verify Password has strong complexity
            let password = u["login"]["password"].as_str().unwrap();
            assert!(password.len() >= 10, "MMO password must be >= 10 chars");
            assert!(password.chars().any(|c| c.is_ascii_uppercase()), "Password must have uppercase");
            assert!(password.chars().any(|c| c.is_ascii_lowercase()), "Password must have lowercase");
            assert!(password.chars().any(|c| c.is_ascii_digit()), "Password must have digits");
            assert!(password.chars().any(|c| "@#$!&*".contains(c)), "Password must have special symbol");

            // Verify Phone has 10 digits
            let phone = u["phone"].as_str().unwrap();
            assert_eq!(phone.len(), 10, "VN phone must have 10 digits");

            // Verify Ward and District are populated
            let ward = u["location"]["ward"].as_str().unwrap();
            let district = u["location"]["district"].as_str().unwrap();
            assert!(!ward.is_empty(), "Ward must not be empty");
            assert!(!district.is_empty(), "District must not be empty");
        }
    }
}
