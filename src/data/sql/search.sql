SELECT name,
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
FROM course
WHERE course MATCH ?1
ORDER BY rank
LIMIT ?2;