use rand::Rng;
use serde_json::{json, Value};
use uuid::Uuid;

pub fn generate_local_users(count: u32, gender_filter: Option<&str>, nat_filter: Option<&str>) -> Value {
    let mut rng = rand::thread_rng();
    let nat = nat_filter.unwrap_or("US").to_uppercase();
    let is_vn = nat.contains("VN");

    let first_names_male = if is_vn {
        vec!["Minh", "Hoàng", "Duy", "Tuấn", "Nam", "Quân", "Long", "Đức", "Anh", "Hùng", "Bảo", "Huy"]
    } else {
        vec!["James", "John", "Robert", "Michael", "William", "David", "Richard", "Joseph", "Thomas", "Charles", "Daniel", "Matthew"]
    };

    let first_names_female = if is_vn {
        vec!["Linh", "Trang", "Hương", "Mai", "Lan", "Ngọc", "Hà", "Phương", "Thu", "Thảo", "Huyền", "Yến"]
    } else {
        vec!["Mary", "Patricia", "Jennifer", "Linda", "Elizabeth", "Barbara", "Susan", "Jessica", "Sarah", "Karen", "Lisa", "Nancy"]
    };

    let last_names = if is_vn {
        vec!["Nguyễn", "Trần", "Lê", "Phạm", "Hoàng", "Huỳnh", "Phan", "Vũ", "Võ", "Đặng", "Bùi", "Đỗ", "Hồ", "Ngô", "Dương"]
    } else {
        vec!["Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis", "Rodriguez", "Martinez", "Hernandez", "Lopez"]
    };

    let cities = if is_vn {
        vec!["Hà Nội", "TP Hồ Chí Minh", "Đà Nẵng", "Hải Phòng", "Cần Thơ", "Nha Trang", "Huế", "Vũng Tàu"]
    } else {
        vec!["New York", "Los Angeles", "Chicago", "Houston", "Phoenix", "Philadelphia", "San Antonio", "San Diego"]
    };

    let streets = if is_vn {
        vec!["Đường Lê Lợi", "Đường Nguyễn Huệ", "Đường Trần Hưng Đạo", "Đường Hai Bà Trưng", "Đường Lý Thường Kiệt", "Đường Phan Chu Trinh"]
    } else {
        vec!["Main Street", "Oak Avenue", "Maple Lane", "Cedar Drive", "Pine Court", "Washington Boulevard"]
    };

    let mut results = Vec::new();

    for _ in 0..count {
        let is_male = match gender_filter {
            Some("male") => true,
            Some("female") => false,
            _ => rng.gen_bool(0.5),
        };

        let gender_str = if is_male { "male" } else { "female" };
        let title_str = if is_male { "Mr" } else { if rng.gen_bool(0.5) { "Ms" } else { "Mrs" } };

        let first = if is_male {
            first_names_male[rng.gen_range(0..first_names_male.len())]
        } else {
            first_names_female[rng.gen_range(0..first_names_female.len())]
        };

        let last = last_names[rng.gen_range(0..last_names.len())];
        let street_name = streets[rng.gen_range(0..streets.len())];
        let street_number = rng.gen_range(10..9999);
        let city = cities[rng.gen_range(0..cities.len())];
        let country = if is_vn { "Vietnam" } else { "United States" };
        let postcode = format!("{:05}", rng.gen_range(10000..99999));

        let age = rng.gen_range(20..65);
        let birth_year = 2026 - age;
        let birth_month = rng.gen_range(1..=12);
        let birth_day = rng.gen_range(1..=28);
        let dob_str = format!("{:04}-{:02}-{:02}T08:00:00.000Z", birth_year, birth_month, birth_day);

        let random_suffix = rng.gen_range(100..9999);
        let username = format!("{}{}{}", first.to_lowercase().replace(' ', ""), last.to_lowercase().replace(' ', ""), random_suffix);
        let email = format!("{}@example.com", username);
        let password = format!("Pass_{:04}!", rng.gen_range(1000..9999));

        let phone_prefix = if is_vn { "09" } else { "555-" };
        let phone = format!("{}{:07}", phone_prefix, rng.gen_range(1000000..9999999));

        let avatar_gender = if is_male { "men" } else { "women" };
        let avatar_id = rng.gen_range(1..99);

        let user = json!({
            "gender": gender_str,
            "name": {
                "title": title_str,
                "first": first,
                "last": last
            },
            "location": {
                "street": {
                    "number": street_number,
                    "name": street_name
                },
                "city": city,
                "state": if is_vn { city } else { "California" },
                "country": country,
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
                "uuid": Uuid::new_v4().to_string(),
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
                "date": "2022-01-01T00:00:00.000Z",
                "age": 4
            },
            "phone": phone.clone(),
            "cell": phone,
            "id": {
                "name": nat,
                "value": format!("{:09}", rng.gen_range(100000000..999999999))
            },
            "picture": {
                "large": format!("https://randomuser.me/api/portraits/{}/{}.jpg", avatar_gender, avatar_id),
                "medium": format!("https://randomuser.me/api/portraits/med/{}/{}.jpg", avatar_gender, avatar_id),
                "thumbnail": format!("https://randomuser.me/api/portraits/thumb/{}/{}.jpg", avatar_gender, avatar_id)
            },
            "nat": nat
        });

        results.push(user);
    }

    json!({
        "results": results,
        "info": {
            "seed": "tauri-rust-generator",
            "results": count,
            "page": 1,
            "version": "2.0-rust-native"
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_default_count() {
        let val = generate_local_users(10, None, None);
        let list = val["results"].as_array().expect("results should be an array");
        assert_eq!(list.len(), 10);
    }

    #[test]
    fn test_generate_vietnamese_users() {
        let val = generate_local_users(5, Some("male"), Some("VN"));
        let list = val["results"].as_array().unwrap();
        assert_eq!(list.len(), 5);
        for u in list {
            assert_eq!(u["gender"], "male");
            assert_eq!(u["nat"], "VN");
            assert_eq!(u["location"]["country"], "Vietnam");
            assert!(u["email"].as_str().unwrap().contains("@example.com"));
            assert!(!u["login"]["password"].as_str().unwrap().is_empty());
        }
    }

    #[test]
    fn test_generate_female_us() {
        let val = generate_local_users(5, Some("female"), Some("US"));
        let list = val["results"].as_array().unwrap();
        assert_eq!(list.len(), 5);
        for u in list {
            assert_eq!(u["gender"], "female");
            assert_eq!(u["nat"], "US");
            assert_eq!(u["location"]["country"], "United States");
        }
    }
}
