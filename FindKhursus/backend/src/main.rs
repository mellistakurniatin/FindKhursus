use ic_cdk::api::{call, trap};
use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Debug)]
struct Course {
    id: u64,
    title: String,
    description: String,
    price: u64,
    category: String,
    provider: Principal,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
struct Transaction {
    course_id: u64,
    buyer: Principal,
}

struct KursusPlatform {
    courses: HashMap<u64, Course>,
    transactions: Vec<Transaction>,
    next_course_id: u64,
}

impl KursusPlatform {
    fn new() -> Self {
        KursusPlatform {
            courses: HashMap::new(),
            transactions: Vec::new(),
            next_course_id: 1,
        }
    }

    fn add_course(&mut self, provider: Principal, title: String, description: String, price: u64, category: String) -> u64 {
        let course = Course {
            id: self.next_course_id,
            title,
            description,
            price,
            category,
            provider,
        };
        self.courses.insert(self.next_course_id, course);
        self.next_course_id += 1;
        self.next_course_id - 1
    }

    fn list_courses(&self, category: Option<String>) -> Vec<Course> {
        self.courses
            .values()
            .filter(|course| {
                if let Some(cat) = &category {
                    &course.category == cat
                } else {
                    true
                }
            })
            .cloned()
            .collect()
    }

    fn buy_course(&mut self, course_id: u64, buyer: Principal) -> Result<(), String> {
        if self.courses.contains_key(&course_id) {
            let transaction = Transaction {
                course_id,
                buyer,
            };
            self.transactions.push(transaction);
            Ok(())
        } else {
            Err("Kursus tidak ditemukan".to_string())
        }
    }
}

static mut KURSUS_PLATFORM: Option<KursusPlatform> = None;

#[ic_cdk::init]
fn init() {
    unsafe {
        KURSUS_PLATFORM = Some(KursusPlatform::new());
    }
}

#[ic_cdk::update]
fn add_course(title: String, description: String, price: u64, category: String) -> u64 {
    let caller = ic_cdk::caller();
    let platform = unsafe { KURSUS_PLATFORM.as_mut().unwrap() };
    platform.add_course(caller, title, description, price, category)
}

#[ic_cdk::query]
fn list_courses(category: Option<String>) -> Vec<Course> {
    let platform = unsafe { KURSUS_PLATFORM.as_ref().unwrap() };
    platform.list_courses(category)
}

#[ic_cdk::update]
fn buy_course(course_id: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();
    let platform = unsafe { KURSUS_PLATFORM.as_mut().unwrap() };
    platform.buy_course(course_id, caller)
}
