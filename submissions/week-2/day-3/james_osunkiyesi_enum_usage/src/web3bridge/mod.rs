
use std::collections::HashMap;
use uuid::Uuid;

pub enum StaffDepartment {
    MediaTeam,
    IT,
    Manager,
    SocialTeam,
    Kitchen,
    TechnicianSuperVisor,
}

struct Staff {
    name: String,
    id: Uuid,
    dept: StaffDepartment,
    is_employed: bool,
}

pub struct Web3Bridge {
    staffs: HashMap<Uuid, Staff>,
}

impl Staff {
    fn new(name: String, dept: StaffDepartment) -> (Uuid, Self) {
        let id = Uuid::new_v4();
        (
            id,
            Self {
                name,
                id,
                dept,
                is_employed: true,
            },
        )
    }

    fn terminate_employment(&mut self) {
        self.is_employed = false
    }

    fn is_employed(&self) -> bool {
        self.is_employed
    }

    fn check_building_access(&self) -> Result<bool, &str> {
        if self.is_employed {
            match self.dept {
                StaffDepartment::MediaTeam => Ok(true),
                StaffDepartment::IT => Ok(true),
                StaffDepartment::Manager => Ok(true),
                _ => Err("No access to the building"),
            }
        } else {
            panic!("Staff has already been removed")
        }
    }
}

impl Web3Bridge {
    pub fn init() -> Self {
        Self {
            staffs: HashMap::new(),
        }
    }
    pub fn employ(&mut self, name: String, staff_department: StaffDepartment) {
        let (id, new_staff) = Staff::new(name, staff_department);
        self.staffs.insert(id, new_staff);
    }

    pub fn terminate_employment(&mut self, id: Uuid) {
        let _staff = self.staffs.get(&id);
        match _staff {
            Some(staff) => {
                if staff.is_employed {
                    self.staffs.entry(id).and_modify(|e| e.is_employed = false);
                } else {
                    panic!("Staff has already been removed")
                }
            }
            None => panic!("Staff does not exist"),
        }
    }

    pub fn enter_building(&self, id: Uuid) {
        let staff = self.staffs.get(&id).unwrap();

        let has_access = staff.check_building_access().unwrap();
        if has_access {
            println!("You shall Enter")
        }
    }

    pub fn get_active_staffs(&self) -> Vec<Uuid> {
        self.staffs
            .iter()
            .filter_map(
                |(id, staff)| {
                    if staff.is_employed { Some(*id) } else { None }
                },
            )
            .collect()
    }
}
