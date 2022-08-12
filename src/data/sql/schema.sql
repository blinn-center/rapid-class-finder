CREATE VIRTUAL TABLE course USING fts5(
    name,
    method,
    crn,
    subject,
    course_number,
    section,
    associated_term,
    registration_start,
    registration_end,
    levels,
    campus,
    schedule_type,
    full_method,
    credits,
    catalog_entry_link
)