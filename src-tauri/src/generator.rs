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

    let last_names_vn = [
        "Nguyễn", "Trần", "Lê", "Phạm", "Hoàng", "Huỳnh", "Phan", "Vũ", "Võ", "Đặng",
        "Bùi", "Đỗ", "Hồ", "Ngô", "Dương", "Lý", "Đinh", "Đoàn", "Lâm", "Mai",
        "Trịnh", "Đào", "Cao", "Hà", "Lưu", "Lương", "Thái", "Châu", "Tạ", "Phùng"
    ];

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

    let cities_vn = [
        "Hà Nội", "TP Hồ Chí Minh", "Đà Nẵng", "Hải Phòng", "Cần Thơ",
        "Nha Trang", "Huế", "Vũng Tàu", "Bình Dương", "Đồng Nai", "Quảng Ninh"
    ];

    let streets_vn = [
        "Đường Lê Lợi", "Đường Nguyễn Huệ", "Đường Trần Hưng Đạo", "Đường Hai Bà Trưng",
        "Đường Lý Thường Kiệt", "Đường Phan Chu Trinh", "Đường Hoàng Hoa Thám", "Đường Điện Biên Phủ",
        "Đường Nguyễn Thị Minh Khai", "Đường Võ Văn Kiệt", "Đường Cách Mạng Tháng 8", "Đường Nam Kỳ Khởi Nghĩa"
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

    let cities_en = [
        "New York", "Los Angeles", "Chicago", "Houston", "Phoenix",
        "Philadelphia", "San Antonio", "San Diego", "Austin", "Seattle"
    ];

    let streets_en = [
        "Main Street", "Oak Avenue", "Maple Lane", "Cedar Drive", "Pine Court",
        "Washington Boulevard", "Lakeview Terrace", "Sunset Way", "Broadway"
    ];

    let jobs_en = [
        "Staff Software Engineer", "Product Manager", "Lead Data Scientist",
        "Cloud Solutions Architect", "UI/UX Principal", "Engineering Director"
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

        let (full_first_name, last_name, job, street_name, city_name, country_name) = if is_vn {
            let last = last_names_vn[rng.gen_range(0..last_names_vn.len())];
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
            let street = streets_vn[rng.gen_range(0..streets_vn.len())];
            let city = cities_vn[rng.gen_range(0..cities_vn.len())];
            (first_combined, last, job, street, city, "Vietnam")
        } else {
            let last = last_names_en[rng.gen_range(0..last_names_en.len())];
            let first = if is_male {
                first_names_male_en[rng.gen_range(0..first_names_male_en.len())]
            } else {
                first_names_female_en[rng.gen_range(0..first_names_female_en.len())]
            };
            let job = jobs_en[rng.gen_range(0..jobs_en.len())];
            let street = streets_en[rng.gen_range(0..streets_en.len())];
            let city = cities_en[rng.gen_range(0..cities_en.len())];
            (first.to_string(), last, job, street, city, "United States")
        };

        let street_number = rng.gen_range(10..999);
        let postcode = if is_vn {
            format!("{:05}", rng.gen_range(70000..75000))
        } else {
            format!("{:05}", rng.gen_range(10000..99999))
        };

        let age = rng.gen_range(22..62);
        let birth_year = 2026 - age;
        let birth_month = rng.gen_range(1..=12);
        let birth_day = rng.gen_range(1..=28);
        let dob_str = format!("{:04}-{:02}-{:02}T08:00:00.000Z", birth_year, birth_month, birth_day);

        let clean_first = to_ascii_slug(&full_first_name).replace('_', "");
        let clean_last = to_ascii_slug(last_name).replace('_', "");
        let random_suffix = rng.gen_range(10..999);
        let username = format!("{}_{}{}", clean_last, clean_first, random_suffix);
        let email = format!("{}.{}{}@tuquet.io", clean_first, clean_last, random_suffix);
        let password = format!("Pass_{:04}!", rng.gen_range(1000..9999));

        let phone_prefix = if is_vn {
            let prefixes = ["090", "091", "098", "097", "032", "070", "079", "083", "088"];
            prefixes[rng.gen_range(0..prefixes.len())]
        } else {
            "555-"
        };
        let phone = format!("{}{:07}", phone_prefix, rng.gen_range(1000000..9999999));

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
            4 => "#e11d48", // Rose
            _ => "#7c3aed", // Violet
        };
        let inline_svg_data_uri = format!(
            "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><rect width='100' height='100' rx='28' fill='{}'/><text x='50%' y='55%' dominant-baseline='middle' text-anchor='middle' fill='white' font-family='system-ui, sans-serif' font-size='36' font-weight='bold'>{}</text></svg>",
            bg_color, initials
        );

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
                "city": city_name,
                "state": if is_vn { city_name } else { "California" },
                "country": country_name,
                "postcode": postcode,
                "coordinates": {
                    "latitude": format!("{:.4}", rng.gen_range(-80.0..80.0)),
                    "longitude": format!("{:.4}", rng.gen_range(-170.0..170.0))
                },
                "timezone": {
                    "offset": "+07:00",
                    "description": "UTC+7"
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
                "name": if is_vn { "CCCD" } else { "SSN" },
                "value": format!("{:012}", rng.gen_range(100000000000_u64..999999999999_u64))
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
    fn test_generate_vietnamese_users() {
        let data = generate_local_users(10, Some("male"), Some("VN"), Some("real"));
        let list = data["results"].as_array().unwrap();
        assert_eq!(list.len(), 10);
        for u in list {
            assert_eq!(u["gender"], "male");
            assert_eq!(u["nat"], "VN");
            assert!(u["job"].is_string());
            assert_eq!(u["id"]["name"], "CCCD");
            let email = u["email"].as_str().unwrap();
            assert!(email.is_ascii(), "Vietnamese email must be strictly ASCII: {}", email);
        }
    }
}
