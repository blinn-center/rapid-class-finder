use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Days {
    pub sunday: bool,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct ScheduledMeetingTime {
    #[serde(rename = "type")]
    pub meeting_time_type: String,
    pub time: String,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    #[serde(rename = "rawDays")]
    pub raw_days: String,
    pub days: Days,
    #[serde(rename = "where")]
    pub location: String,
    #[serde(rename = "dateRange")]
    pub date_range: String,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "scheduleType")]
    pub schedule_type: String,
    pub instructors: String,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Course {
    pub name: String,
    pub method: String,
    pub crn: String,
    pub subject: String,
    #[serde(rename = "courseNumber")]
    pub course_number: String,
    pub section: String,

    #[serde(rename = "associatedTerm")]
    pub associated_term: String,
    #[serde(rename = "registrationStart")]
    pub registration_start: String,
    #[serde(rename = "registrationEnd")]
    pub registration_end: String,
    pub levels: String,

    pub campus: String,
    #[serde(rename = "scheduleType")]
    pub schedule_type: String,
    #[serde(rename = "fullMethod")]
    pub full_method: String,
    pub credits: String,

    #[serde(rename = "catalogEntryLink")]
    pub catalog_entry_link: Option<String>,

    #[serde(rename = "scheduledMeetingTimes")]
    pub scheduled_meeting_times: Vec<ScheduledMeetingTime>,
}

#[derive(Serialize, Default, Debug)]
pub struct SqlCourse {
    pub name: Option<String>,
    pub method: Option<String>,
    pub crn: Option<String>,
    pub subject: Option<String>,
    pub course_number: Option<String>,
    pub section: Option<String>,
    pub associated_term: Option<String>,
    pub registration_start: Option<String>,
    pub registration_end: Option<String>,
    pub levels: Option<String>,
    pub campus: Option<String>,
    pub schedule_type: Option<String>,
    pub full_method: Option<String>,
    pub credits: Option<String>,
    pub catalog_entry_link: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Subject {
    pub name: String,
    pub id: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct Term {
    pub subjects: Vec<Subject>,
    pub courses: Vec<Course>,
}

#[derive(Deserialize, Default, Debug)]
pub struct Everything {
    pub terms: Vec<Term>,
}

impl Everything {
    pub fn course_iter(&self) -> impl Iterator<Item = &Course> {
        self.terms.iter().flat_map(|term| &term.courses)
    }
}
