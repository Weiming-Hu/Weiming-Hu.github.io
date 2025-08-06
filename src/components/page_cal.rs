use dioxus::prelude::*;
use super::css_preset::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug, PartialEq)]
struct CalendarEvent {
    start: String,
    end: String,
    is_busy: bool,
    // Add parsed datetime components for easier comparison
    start_date: String, // YYYYMMDD format
    start_hour: i32,
    start_minute: i32,
    end_hour: i32,
    end_minute: i32,
}

#[derive(Clone, Debug, PartialEq)]
struct RecurrenceRule {
    freq: String,           // DAILY, WEEKLY, MONTHLY, YEARLY
    interval: i32,          // How often to repeat (e.g., every 2 weeks)
    by_day: Vec<String>,    // Days of week (MO, TU, WE, etc.)
    until: Option<String>,  // End date (YYYYMMDD format)
    count: Option<i32>,     // Number of occurrences
}

async fn fetch_calendar_data(urls: Vec<&str>, max_weeks_ahead: i32) -> Result<Vec<CalendarEvent>, String> {
    let mut all_events = Vec::new();
    
    for url in urls {
        match reqwest::get(url).await {
            Ok(response) => {
                if let Ok(text) = response.text().await {
                    // Handle different CORS proxy response formats
                    let ics_content = if url.contains("allorigins.win") {
                        // Parse JSON response from allorigins
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                            json["contents"].as_str().unwrap_or("").to_string()
                        } else {
                            text
                        }
                    } else {
                        // corsproxy.io returns content directly
                        text
                    };
                    
                    // Simple ICS parser for VEVENT blocks
                    let events = parse_ics_busy_free(&ics_content, max_weeks_ahead);
                    
                    // Debug: Log the first few lines of ICS content and parsed events
                    // web_sys::console::log_1(&format!("ICS content preview: {}", 
                    //     ics_content.lines().take(10).collect::<Vec<_>>().join("\\n")).into());
                    // web_sys::console::log_1(&format!("Parsed {} events", events.len()).into());
                    
                    // if !events.is_empty() {
                    //     web_sys::console::log_1(&format!("First event: {:?}", events[0]).into());
                    // }
                    
                    all_events.extend(events);
                }
            }
            Err(e) => {
                return Err(format!("Failed to fetch calendar: {}", e));
            }
        }
    }
    
    Ok(all_events)
}

fn parse_ics_busy_free(ics_content: &str, max_weeks_ahead: i32) -> Vec<CalendarEvent> {
    let mut events = Vec::new();
    let lines: Vec<&str> = ics_content.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        if lines[i].starts_with("BEGIN:VEVENT") {
            let mut start = String::new();
            let mut end = String::new();
            let mut is_all_day = false;
            let mut rrule: Option<String> = None;
            
            // Look for DTSTART, DTEND, and RRULE within this event
            i += 1;
            while i < lines.len() && !lines[i].starts_with("END:VEVENT") {
                if lines[i].starts_with("DTSTART") {
                    start = lines[i].split(':').nth(1).unwrap_or("").to_string();
                    // Check if it's an all-day event (VALUE=DATE parameter)
                    if lines[i].contains("VALUE=DATE") {
                        is_all_day = true;
                    }
                } else if lines[i].starts_with("DTEND") {
                    end = lines[i].split(':').nth(1).unwrap_or("").to_string();
                } else if lines[i].starts_with("RRULE:") {
                    rrule = Some(lines[i].split(':').nth(1).unwrap_or("").to_string());
                }
                i += 1;
            }
            
            // Skip all-day events - they don't conflict with hourly meetings
            if is_all_day {
                continue;
            }
            
            if !start.is_empty() && !end.is_empty() {
                // Store raw ICS datetime strings - we'll parse them with timezone later
                // For now, just do basic validation
                let has_valid_format = start.len() >= 15 && end.len() >= 15;
                
                if has_valid_format {
                    // Generate base event
                    let base_event = CalendarEvent {
                        start: start.clone(),
                        end: end.clone(),
                        is_busy: true,
                        start_date: String::new(), // Will be filled when re-parsing with timezone
                        start_hour: 0,
                        start_minute: 0,
                        end_hour: 0,
                        end_minute: 0,
                    };
                    
                    // Generate recurrence instances if RRULE exists
                    if let Some(rrule_str) = rrule {
                        // web_sys::console::log_1(&format!("Found RRULE: {} for event starting at {}", rrule_str, start).into());
                        let generated_events = generate_recurring_events(&base_event, &rrule_str, max_weeks_ahead);
                        // web_sys::console::log_1(&format!("Generated {} recurring events", generated_events.len()).into());
                        events.extend(generated_events);
                    } else {
                        // Single event
                        events.push(base_event);
                    }
                }
            }
        }
        i += 1;
    }
    
    events
}

// Parse RRULE string into RecurrenceRule struct
fn parse_rrule(rrule_str: &str) -> Option<RecurrenceRule> {
    // web_sys::console::log_1(&format!("Parsing RRULE: {}", rrule_str).into());
    
    let mut freq = String::new();
    let mut interval = 1;
    let mut by_day = Vec::new();
    let mut until = None;
    let mut count = None;
    
    // Parse RRULE parameters
    for param in rrule_str.split(';') {
        if let Some((key, value)) = param.split_once('=') {
            match key {
                "FREQ" => freq = value.to_string(),
                "INTERVAL" => interval = value.parse().unwrap_or(1),
                "BYDAY" => {
                    by_day = value.split(',').map(|s| s.to_string()).collect();
                    // web_sys::console::log_1(&format!("Found BYDAY: {:?}", by_day).into());
                }
                "UNTIL" => {
                    // UNTIL can be in YYYYMMDDTHHMMSSZ or YYYYMMDD format
                    let until_date = if value.contains('T') {
                        value.split('T').next().unwrap_or(value)
                    } else {
                        value
                    };
                    until = Some(until_date.to_string());
                }
                "COUNT" => count = value.parse().ok(),
                _ => {} // Ignore other parameters for now
            }
        }
    }
    
    if !freq.is_empty() {
        let rule = RecurrenceRule {
            freq: freq.clone(),
            interval,
            by_day: by_day.clone(),
            until: until.clone(),
            count,
        };
        // web_sys::console::log_1(&format!("Parsed RRULE: freq={}, interval={}, by_day={:?}, until={:?}, count={:?}", 
        //     freq, interval, by_day, until, count).into());
        Some(rule)
    } else {
        None
    }
}

// Generate recurring events based on RRULE
fn generate_recurring_events(base_event: &CalendarEvent, rrule_str: &str, max_weeks_ahead: i32) -> Vec<CalendarEvent> {
    // Parse the RRULE
    let Some(rrule) = parse_rrule(rrule_str) else {
        // If RRULE parsing fails, return just the base event
        return vec![base_event.clone()];
    };
    
    // Parse base event start time
    let base_start = &base_event.start;
    let base_end = &base_event.end;
    
    // Extract date and time from base event (format: YYYYMMDDTHHMMSS or YYYYMMDDTHHMMSSZ)
    let clean_start = base_start.replace("T", "").replace("Z", "");
    if clean_start.len() < 8 {
        return vec![base_event.clone()];
    }
    
    let base_year = clean_start[0..4].parse::<i32>().unwrap_or(2024);
    let base_month = clean_start[4..6].parse::<i32>().unwrap_or(1);
    let base_day = clean_start[6..8].parse::<i32>().unwrap_or(1);
    
    // Calculate duration in minutes
    let duration_minutes = calculate_event_duration_minutes(base_start, base_end);
    
    // Generate events based on frequency
    match rrule.freq.as_str() {
        "DAILY" => generate_daily_events(base_event, &rrule, base_year, base_month, base_day, duration_minutes, max_weeks_ahead),
        "WEEKLY" => generate_weekly_events(base_event, &rrule, base_year, base_month, base_day, duration_minutes, max_weeks_ahead),
        "MONTHLY" => generate_monthly_events(base_event, &rrule, base_year, base_month, base_day, duration_minutes, max_weeks_ahead),
        "YEARLY" => generate_yearly_events(base_event, &rrule, base_year, base_month, base_day, duration_minutes, max_weeks_ahead),
        _ => {
            // Unknown frequency, return base event
            vec![base_event.clone()]
        }
    }
}

// Calculate duration between start and end times in minutes
fn calculate_event_duration_minutes(start: &str, end: &str) -> i32 {
    let clean_start = start.replace("T", "").replace("Z", "");
    let clean_end = end.replace("T", "").replace("Z", "");
    
    if clean_start.len() >= 12 && clean_end.len() >= 12 {
        let start_hour = clean_start[8..10].parse::<i32>().unwrap_or(0);
        let start_minute = clean_start[10..12].parse::<i32>().unwrap_or(0);
        let end_hour = clean_end[8..10].parse::<i32>().unwrap_or(0);
        let end_minute = clean_end[10..12].parse::<i32>().unwrap_or(0);
        
        let start_total_minutes = start_hour * 60 + start_minute;
        let end_total_minutes = end_hour * 60 + end_minute;
        
        end_total_minutes - start_total_minutes
    } else {
        60 // Default to 1 hour
    }
}

// Generate daily recurring events
fn generate_daily_events(base_event: &CalendarEvent, rrule: &RecurrenceRule, base_year: i32, base_month: i32, base_day: i32, duration_minutes: i32, max_weeks_ahead: i32) -> Vec<CalendarEvent> {
    let mut events = vec![base_event.clone()]; // Include the original event
    
    // Limit based on max generation date - enough to cover larger max_weeks_ahead values
    let max_occurrences = rrule.count.unwrap_or(200); // Max ~6 months of daily events
    let max_date = get_max_generation_date(max_weeks_ahead);
    
    for i in 1..max_occurrences {
        let days_offset = i * rrule.interval;
        
        // Calculate new date using JavaScript Date API for proper date arithmetic
        if let Some((new_year, new_month, new_day)) = add_days_to_date(base_year, base_month, base_day, days_offset) {
            let new_date_str = format!("{:04}{:02}{:02}", new_year, new_month, new_day);
            
            // Check against UNTIL date
            if let Some(until_date) = &rrule.until {
                if new_date_str > *until_date {
                    break;
                }
            }
            
            // Check against maximum generation date
            if new_date_str > max_date {
                break;
            }
            
            // Generate new event
            if let Some(new_event) = create_recurring_event_instance(base_event, new_year, new_month, new_day, duration_minutes) {
                events.push(new_event);
            }
        }
    }
    
    events
}

// Generate weekly recurring events
fn generate_weekly_events(base_event: &CalendarEvent, rrule: &RecurrenceRule, base_year: i32, base_month: i32, base_day: i32, duration_minutes: i32, max_weeks_ahead: i32) -> Vec<CalendarEvent> {
    let mut events = vec![base_event.clone()]; // Include the original event
    
    // web_sys::console::log_1(&format!("Generating weekly events for base date {}/{}/{}, BYDAY: {:?}", 
    //     base_year, base_month, base_day, rrule.by_day).into());
    
    // If BYDAY is specified, handle multiple days per week
    if !rrule.by_day.is_empty() {
        // Get the base event's day of week
        let base_weekday = get_weekday_from_date(base_year, base_month, base_day);
        // web_sys::console::log_1(&format!("Base event weekday: {} (date: {}/{}/{})", base_weekday, base_year, base_month, base_day).into());
        
        // For each BYDAY value, generate events
        for target_day in &rrule.by_day {
            // web_sys::console::log_1(&format!("Processing BYDAY: {}", target_day).into());
            
            // Convert BYDAY to weekday number (0=Sunday, 6=Saturday)
            let target_weekday = match target_day.as_str() {
                "SU" => 0,
                "MO" => 1,
                "TU" => 2,
                "WE" => 3,
                "TH" => 4,
                "FR" => 5,
                "SA" => 6,
                _ => continue, // Skip unknown day
            };
            
            // web_sys::console::log_1(&format!("Target weekday: {} ({})", target_weekday, target_day).into());
            
            // If this is not the base event's day, create the first occurrence for this day
            if target_weekday != base_weekday {
                let days_offset = if target_weekday > base_weekday {
                    target_weekday - base_weekday
                } else {
                    7 - (base_weekday - target_weekday)
                };
                
                // web_sys::console::log_1(&format!("Days offset for {}: {}", target_day, days_offset).into());
                
                if let Some((first_year, first_month, first_day)) = add_days_to_date(base_year, base_month, base_day, days_offset) {
                    // web_sys::console::log_1(&format!("First occurrence for {}: {}/{}/{}", target_day, first_year, first_month, first_day).into());
                    
                    if let Some(first_event) = create_recurring_event_instance(base_event, first_year, first_month, first_day, duration_minutes) {
                        // web_sys::console::log_1(&format!("Added {} first occurrence: {}/{}/{} ({})", target_day, first_year, first_month, first_day, first_event.start).into());
                        events.push(first_event);
                    }
                    
                    // Generate subsequent weekly occurrences for this day
                    let max_occurrences = rrule.count.unwrap_or(30); // Support larger max_weeks_ahead values
                    let max_date = get_max_generation_date(max_weeks_ahead);
                    
                    for i in 1..max_occurrences {
                        let weeks_offset = i * rrule.interval;
                        let total_days_offset = days_offset + (weeks_offset * 7);
                        
                        if let Some((new_year, new_month, new_day)) = add_days_to_date(base_year, base_month, base_day, total_days_offset) {
                            let new_date_str = format!("{:04}{:02}{:02}", new_year, new_month, new_day);
                            
                            // Check against UNTIL date
                            if let Some(until_date) = &rrule.until {
                                if new_date_str > *until_date {
                                    break;
                                }
                            }
                            
                            // Check against maximum generation date
                            if new_date_str > max_date {
                                break;
                            }
                            
                            if let Some(new_event) = create_recurring_event_instance(base_event, new_year, new_month, new_day, duration_minutes) {
                                // web_sys::console::log_1(&format!("Added {} occurrence: {}/{}/{}", target_day, new_year, new_month, new_day).into());
                                events.push(new_event);
                            }
                        }
                    }
                }
            } else {
                // This is the base event's day, generate additional weekly occurrences
                let max_occurrences = rrule.count.unwrap_or(30); // Support larger max_weeks_ahead values
                let max_date = get_max_generation_date(max_weeks_ahead);
                
                for i in 1..max_occurrences {
                    let weeks_offset = i * rrule.interval;
                    let days_offset = weeks_offset * 7;
                    
                    if let Some((new_year, new_month, new_day)) = add_days_to_date(base_year, base_month, base_day, days_offset) {
                        let new_date_str = format!("{:04}{:02}{:02}", new_year, new_month, new_day);
                        
                        // Check against UNTIL date
                        if let Some(until_date) = &rrule.until {
                            if new_date_str > *until_date {
                                break;
                            }
                        }
                        
                        // Check against maximum generation date
                        if new_date_str > max_date {
                            break;
                        }
                        
                        if let Some(new_event) = create_recurring_event_instance(base_event, new_year, new_month, new_day, duration_minutes) {
                            // web_sys::console::log_1(&format!("Added {} base day occurrence: {}/{}/{}", target_day, new_year, new_month, new_day).into());
                            events.push(new_event);
                        }
                    }
                }
            }
        }
    } else {
        // No BYDAY specified, generate simple weekly recurrence
        let max_occurrences = rrule.count.unwrap_or(30); // Support larger max_weeks_ahead values
        let max_date = get_max_generation_date(max_weeks_ahead);
        
        for i in 1..max_occurrences {
            let weeks_offset = i * rrule.interval;
            let days_offset = weeks_offset * 7;
            
            // Calculate new date
            if let Some((new_year, new_month, new_day)) = add_days_to_date(base_year, base_month, base_day, days_offset) {
                let new_date_str = format!("{:04}{:02}{:02}", new_year, new_month, new_day);
                
                // Check against UNTIL date
                if let Some(until_date) = &rrule.until {
                    if new_date_str > *until_date {
                        break;
                    }
                }
                
                // Check against maximum generation date
                if new_date_str > max_date {
                    break;
                }
                
                // Generate new event
                if let Some(new_event) = create_recurring_event_instance(base_event, new_year, new_month, new_day, duration_minutes) {
                    events.push(new_event);
                }
            }
        }
    }
    
    // web_sys::console::log_1(&format!("Generated {} total weekly events", events.len()).into());
    events
}

// Generate monthly recurring events (simplified)
fn generate_monthly_events(base_event: &CalendarEvent, rrule: &RecurrenceRule, base_year: i32, base_month: i32, base_day: i32, duration_minutes: i32, max_weeks_ahead: i32) -> Vec<CalendarEvent> {
    let mut events = vec![base_event.clone()]; // Include the original event
    
    // Limit to cover larger max_weeks_ahead values
    let max_occurrences = rrule.count.unwrap_or(8); // Max ~6 months of monthly events
    let max_date = get_max_generation_date(max_weeks_ahead);
    
    for i in 1..max_occurrences {
        let months_offset = i * rrule.interval;
        let new_month = base_month + months_offset;
        let mut new_year = base_year;
        let mut adjusted_month = new_month;
        
        // Handle year overflow
        while adjusted_month > 12 {
            adjusted_month -= 12;
            new_year += 1;
        }
        
        let new_date_str = format!("{:04}{:02}{:02}", new_year, adjusted_month, base_day);
        
        // Check against UNTIL date
        if let Some(until_date) = &rrule.until {
            if new_date_str > *until_date {
                break;
            }
        }
        
        // Check against maximum generation date
        if new_date_str > max_date {
            break;
        }
        
        // Generate new event (assuming same day of month exists)
        if let Some(new_event) = create_recurring_event_instance(base_event, new_year, adjusted_month, base_day, duration_minutes) {
            events.push(new_event);
        }
    }
    
    events
}

// Generate yearly recurring events
fn generate_yearly_events(base_event: &CalendarEvent, rrule: &RecurrenceRule, base_year: i32, base_month: i32, base_day: i32, duration_minutes: i32, max_weeks_ahead: i32) -> Vec<CalendarEvent> {
    let mut events = vec![base_event.clone()]; // Include the original event
    
    // Generate a few yearly events to support longer max_weeks_ahead values
    let max_occurrences = rrule.count.unwrap_or(3); // Max 3 yearly events (current + next 2 years)
    let max_date = get_max_generation_date(max_weeks_ahead);
    
    for i in 1..max_occurrences {
        let years_offset = i * rrule.interval;
        let new_year = base_year + years_offset;
        
        let new_date_str = format!("{:04}{:02}{:02}", new_year, base_month, base_day);
        
        // Check against UNTIL date
        if let Some(until_date) = &rrule.until {
            if new_date_str > *until_date {
                break;
            }
        }
        
        // Check against maximum generation date
        if new_date_str > max_date {
            break;
        }
        
        // Generate new event
        if let Some(new_event) = create_recurring_event_instance(base_event, new_year, base_month, base_day, duration_minutes) {
            events.push(new_event);
        }
    }
    
    events
}

// Helper function to add days to a date using JavaScript Date API
fn add_days_to_date(year: i32, month: i32, day: i32, days_to_add: i32) -> Option<(i32, i32, i32)> {
    let js_code = format!(
        r#"(() => {{
            const date = new Date({}, {}, {});
            date.setDate(date.getDate() + {});
            return JSON.stringify({{
                year: date.getFullYear(),
                month: date.getMonth() + 1,
                day: date.getDate()
            }});
        }})()"#,
        year, month - 1, day, days_to_add // month is 0-based in JS
    );
    
    if let Ok(result) = js_sys::eval(&js_code) {
        if let Some(json_str) = result.as_string() {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json_str) {
                let new_year = parsed["year"].as_i64().unwrap_or(year as i64) as i32;
                let new_month = parsed["month"].as_i64().unwrap_or(month as i64) as i32;
                let new_day = parsed["day"].as_i64().unwrap_or(day as i64) as i32;
                return Some((new_year, new_month, new_day));
            }
        }
    }
    
    None
}

// Helper function to get weekday from date (0=Sunday, 6=Saturday)
fn get_weekday_from_date(year: i32, month: i32, day: i32) -> i32 {
    let js_code = format!(
        r#"(() => {{
            const date = new Date({}, {}, {});
            return date.getDay(); // 0=Sunday, 1=Monday, etc. (already Sunday-first)
        }})()"#,
        year, month - 1, day // month is 0-based in JS
    );
    
    if let Ok(result) = js_sys::eval(&js_code) {
        if let Some(num) = result.as_f64() {
            return num as i32;
        }
    }
    
    0 // Fallback to Sunday
}

// Create a new event instance for a specific date
fn create_recurring_event_instance(base_event: &CalendarEvent, year: i32, month: i32, day: i32, duration_minutes: i32) -> Option<CalendarEvent> {
    // Extract time from base event
    let base_start = &base_event.start;
    let clean_start = base_start.replace("T", "").replace("Z", "");
    
    if clean_start.len() >= 12 {
        let hour = &clean_start[8..10];
        let minute = &clean_start[10..12];
        let second = if clean_start.len() >= 14 { &clean_start[12..14] } else { "00" };
        
        // Create new start time
        let new_start = if base_start.ends_with('Z') {
            format!("{:04}{:02}{:02}T{}{}{}Z", year, month, day, hour, minute, second)
        } else {
            format!("{:04}{:02}{:02}T{}{}{}", year, month, day, hour, minute, second)
        };
        
        // Calculate end time
        let start_hour = hour.parse::<i32>().unwrap_or(0);
        let start_minute = minute.parse::<i32>().unwrap_or(0);
        let total_start_minutes = start_hour * 60 + start_minute;
        let total_end_minutes = total_start_minutes + duration_minutes;
        
        let end_hour = total_end_minutes / 60;
        let end_minute = total_end_minutes % 60;
        
        let new_end = if base_event.end.ends_with('Z') {
            format!("{:04}{:02}{:02}T{:02}{:02}{}Z", year, month, day, end_hour, end_minute, second)
        } else {
            format!("{:04}{:02}{:02}T{:02}{:02}{}", year, month, day, end_hour, end_minute, second)
        };
        
        Some(CalendarEvent {
            start: new_start,
            end: new_end,
            is_busy: base_event.is_busy,
            start_date: String::new(),
            start_hour: 0,
            start_minute: 0,
            end_hour: 0,
            end_minute: 0,
        })
    } else {
        None
    }
}

// Get maximum date for generating recurring events based on max_weeks_ahead
// This calculates to the end of the target week (Saturday) rather than just max_weeks_ahead * 7 days from today
fn get_max_generation_date(max_weeks_ahead: i32) -> String {
    let js_code = format!(r#"(() => {{
        const now = new Date();
        const currentDayOfWeek = now.getDay(); // 0=Sunday, 1=Monday, etc.
        
        // Calculate days to reach the beginning of the target week (Sunday)
        const daysToTargetWeekStart = ({} * 7) - currentDayOfWeek;
        
        // Add 6 more days to reach the end of that week (Saturday)
        const daysToTargetWeekEnd = daysToTargetWeekStart + 6;
        
        const maxDate = new Date(now.getFullYear(), now.getMonth(), now.getDate() + daysToTargetWeekEnd);
        return maxDate.getFullYear().toString() + 
               (maxDate.getMonth() + 1).toString().padStart(2, '0') + 
               maxDate.getDate().toString().padStart(2, '0');
    }})()"#, max_weeks_ahead);
    
    if let Ok(result) = js_sys::eval(&js_code) {
        if let Some(date_str) = result.as_string() {
            return date_str;
        }
    }
    
    // Fallback: max_weeks_ahead to end of target week
    let js_fallback = format!(r#"(() => {{
        const now = new Date();
        const currentDayOfWeek = now.getDay(); // 0=Sunday, 1=Monday, etc.
        
        // Calculate days to reach the beginning of the target week (Sunday)
        const daysToTargetWeekStart = ({} * 7) - currentDayOfWeek;
        
        // Add 6 more days to reach the end of that week (Saturday)
        const daysToTargetWeekEnd = daysToTargetWeekStart + 6;
        
        const fallbackDate = new Date(now.getFullYear(), now.getMonth(), now.getDate() + daysToTargetWeekEnd);
        return fallbackDate.getFullYear().toString() + 
               (fallbackDate.getMonth() + 1).toString().padStart(2, '0') + 
               fallbackDate.getDate().toString().padStart(2, '0');
    }})()"#, max_weeks_ahead);
    
    if let Ok(result) = js_sys::eval(&js_fallback) {
        if let Some(date_str) = result.as_string() {
            return date_str;
        }
    }
    
    // Final fallback: static date (end of 2026 to be safe)
    "20261231".to_string()
}

fn parse_ics_datetime_with_timezone(datetime_str: &str, target_timezone: &str) -> (String, i32, i32) {
    // Handle ICS datetime formats like "20240806T140000Z" or "20240806T140000"
    let clean_datetime = datetime_str.replace("T", "");
    let is_utc = datetime_str.ends_with("Z");
    
    if clean_datetime.len() >= 12 {
        let date_part = if is_utc {
            &clean_datetime[0..8] // YYYYMMDD
        } else {
            &clean_datetime[0..8]
        };
        let time_part = if is_utc {
            &clean_datetime[8..clean_datetime.len()-1] // Remove Z
        } else {
            &clean_datetime[8..]
        };
        
        if time_part.len() >= 4 {
            let hour = time_part[0..2].parse::<i32>().unwrap_or(0);
            let minute = if time_part.len() >= 4 {
                time_part[2..4].parse::<i32>().unwrap_or(0)
            } else {
                0
            };
            
            if is_utc {
                // Convert UTC to target timezone - create a proper UTC date and convert
                let year = date_part[0..4].parse::<i32>().unwrap_or(2024);
                let month = date_part[4..6].parse::<i32>().unwrap_or(1);
                let day = date_part[6..8].parse::<i32>().unwrap_or(1);
                
                // Create a UTC timestamp and convert to target timezone
                let js_code = format!(
                    r#"(() => {{
                        // Create UTC date from ICS datetime
                        const utcDate = new Date(Date.UTC({}, {}, {}, {}, {}));
                        
                        // Convert to target timezone
                        const formatter = new Intl.DateTimeFormat('en-CA', {{
                            timeZone: '{}',
                            year: 'numeric',
                            month: '2-digit', 
                            day: '2-digit',
                            hour: '2-digit',
                            minute: '2-digit',
                            hour12: false
                        }});
                        
                        const parts = formatter.formatToParts(utcDate);
                        const result = {{}};
                        parts.forEach(part => result[part.type] = part.value);
                        
                        console.log('UTC Input:', {}, {}, {}, '{}:{}');
                        console.log('Target TZ:', '{}');
                        console.log('Converted:', result);
                        
                        return JSON.stringify(result);
                    }})()"#,
                    year, month - 1, day, hour, minute, // Date.UTC (month is 0-based in JS)
                    target_timezone,
                    year, month, day, hour, minute, // Console log
                    target_timezone
                );
                
                if let Ok(result) = js_sys::eval(&js_code) {
                    if let Some(json_str) = result.as_string() {
                        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json_str) {
                            let tz_year = parsed["year"].as_str().unwrap_or("2024").parse::<i32>().unwrap_or(2024);
                            let tz_month = parsed["month"].as_str().unwrap_or("01").parse::<i32>().unwrap_or(1);
                            let tz_day = parsed["day"].as_str().unwrap_or("01").parse::<i32>().unwrap_or(1);
                            let tz_hour = parsed["hour"].as_str().unwrap_or("00").parse::<i32>().unwrap_or(0);
                            let tz_minute = parsed["minute"].as_str().unwrap_or("00").parse::<i32>().unwrap_or(0);
                            
                            let tz_date_str = format!("{:04}{:02}{:02}", tz_year, tz_month, tz_day);
                            
                            // Debug log the conversion result
                            // web_sys::console::log_1(&format!("Timezone conversion: {} -> {} {} {}:{:02} ({})", 
                            //     datetime_str, tz_date_str, target_timezone, tz_hour, tz_minute, target_timezone).into());
                            
                            return (tz_date_str, tz_hour, tz_minute);
                        }
                    }
                }
                
                // Fallback: assume local timezone offset
                // web_sys::console::log_1(&format!("Timezone conversion failed for {}, using fallback", datetime_str).into());
                return (date_part.to_string(), hour, minute);
            } else {
                // Not UTC - these are likely in the browser's local timezone
                // Non-UTC ICS events are typically in the local timezone where the calendar was accessed
                let browser_timezone = get_browser_timezone();
                
                if target_timezone == browser_timezone {
                    // No conversion needed - already in target timezone
                    // web_sys::console::log_1(&format!("Non-UTC event, no conversion needed: {} (browser: {})", datetime_str, browser_timezone).into());
                    return (date_part.to_string(), hour, minute);
                } else {
                    // Convert from browser timezone to target timezone
                    let year = date_part[0..4].parse::<i32>().unwrap_or(2024);
                    let month = date_part[4..6].parse::<i32>().unwrap_or(1);
                    let day = date_part[6..8].parse::<i32>().unwrap_or(1);
                    
                    let js_code = format!(
                        r#"(() => {{
                            // For non-UTC events, assume they're in browser timezone and convert to target timezone
                            // Create a date in browser timezone, then get its UTC equivalent, then convert to target timezone
                            const browserDate = new Date({}, {}, {}, {}, {});
                            
                            // Convert this browser time to target timezone
                            const formatter = new Intl.DateTimeFormat('en-CA', {{
                                timeZone: '{}',
                                year: 'numeric',
                                month: '2-digit', 
                                day: '2-digit',
                                hour: '2-digit',
                                minute: '2-digit',
                                hour12: false
                            }});
                            
                            const parts = formatter.formatToParts(browserDate);
                            const result = {{}};
                            parts.forEach(part => result[part.type] = part.value);
                            
                            console.log('Non-UTC conversion:', '{}', 'from browser TZ ({}) to', '{}', '->', JSON.stringify(result));
                            
                            return JSON.stringify(result);
                        }})()"#,
                        year, month - 1, day, hour, minute, // browserDate (month is 0-based in JS)
                        target_timezone,
                        datetime_str, browser_timezone, target_timezone
                    );
                    
                    if let Ok(result) = js_sys::eval(&js_code) {
                        if let Some(json_str) = result.as_string() {
                            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json_str) {
                                let tz_year = parsed["year"].as_str().unwrap_or("2024").parse::<i32>().unwrap_or(2024);
                                let tz_month = parsed["month"].as_str().unwrap_or("01").parse::<i32>().unwrap_or(1);
                                let tz_day = parsed["day"].as_str().unwrap_or("01").parse::<i32>().unwrap_or(1);
                                let tz_hour = parsed["hour"].as_str().unwrap_or("00").parse::<i32>().unwrap_or(0);
                                let tz_minute = parsed["minute"].as_str().unwrap_or("00").parse::<i32>().unwrap_or(0);
                                
                                let tz_date_str = format!("{:04}{:02}{:02}", tz_year, tz_month, tz_day);
                                
                                // Debug log the conversion result
                                // web_sys::console::log_1(&format!("Non-UTC conversion result: {} -> {} {} {}:{:02} ({}->{})", 
                                //     datetime_str, tz_date_str, target_timezone, tz_hour, tz_minute, browser_timezone, target_timezone).into());
                                
                                return (tz_date_str, tz_hour, tz_minute);
                            }
                        }
                    }
                    
                    // Fallback: assume same timezone (no conversion)
                    // web_sys::console::log_1(&format!("Non-UTC conversion failed for {}, using no conversion", datetime_str).into());
                    return (date_part.to_string(), hour, minute);
                }
            }
        }
    }
    
    // Fallback
    ("".to_string(), 0, 0)
}

fn parse_ics_datetime(datetime_str: &str) -> (String, i32, i32) {
    // Use browser timezone as default
    let browser_tz = get_browser_timezone();
    parse_ics_datetime_with_timezone(datetime_str, &browser_tz)
}

// Get the user's browser timezone
fn get_browser_timezone() -> String {
    // Use JavaScript Intl API to get timezone
    let timezone = js_sys::eval("Intl.DateTimeFormat().resolvedOptions().timeZone")
        .unwrap_or_else(|_| js_sys::JsString::from("UTC").into());
    
    timezone.as_string().unwrap_or_else(|| "UTC".to_string())
}

// Get a more user-friendly timezone display name
fn get_timezone_display(timezone: &str) -> String {
    // Try to get a more user-friendly display name
    let display_result = js_sys::eval(&format!(
        "try {{ new Intl.DateTimeFormat('en', {{ timeZone: '{}', timeZoneName: 'short' }}).formatToParts(new Date()).find(part => part.type === 'timeZoneName')?.value || '{}' }} catch(e) {{ '{}' }}", 
        timezone, timezone, timezone
    ));
    
    if let Ok(display) = display_result {
        if let Some(display_str) = display.as_string() {
            return format!("{} ({})", display_str, timezone);
        }
    }
    
    timezone.to_string()
}

#[component]
fn CalendarGrid(events: Vec<CalendarEvent>, weeks_ahead: i32, timezone: String, current_week_offset: i32, start_hour: i32, end_hour: i32) -> Element {
    // Simple week generation - show days of the week with proper dates
    let week_days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    
    // Pre-calculate all dates for the week to avoid repeated JavaScript calls
    let week_dates: Vec<String> = (0..7).map(|day_idx| {
        let target_date = get_date_for_day_with_timezone(day_idx, current_week_offset, &timezone);
        // Extract day from YYYYMMDD format
        if target_date.len() >= 8 {
            let month = &target_date[4..6];
            let day = &target_date[6..8];
            format!("{}/{}", month, day)
        } else {
            format!("8/{}", day_idx + 1)
        }
    }).collect();
    
    // Pre-calculate which days are in the past to avoid repeated JavaScript calls
    let past_days: Vec<bool> = (0..7).map(|day_idx| {
        is_in_past_with_timezone(day_idx, current_week_offset, &timezone)
    }).collect();
    
    // Pre-calculate target dates for each day to avoid repeated JavaScript calls
    let target_dates: Vec<String> = (0..7).map(|day_idx| {
        get_date_for_day_with_timezone(day_idx, current_week_offset, &timezone)
    }).collect();

    // Calculate total 5-minute slots
    let total_hours = end_hour - start_hour;
    let total_slots = total_hours * 12; // 12 five-minute slots per hour

    rsx! {
        div {
            class: "border border-gray-300 rounded-lg overflow-hidden relative", // Add relative positioning for time labels
            
            // Calendar header with days (sticky)
            div {
                class: "bg-gray-100 border-b sticky top-0 z-30",
                div {
                    class: "grid grid-cols-8 gap-0", // 8 columns: time + 7 days
                    
                    // Time column header
                    div {
                        class: "p-2 text-xs font-medium text-gray-700 border-r bg-gray-200",
                        "Time"
                    }
                    
                    // Day headers
                    for (i, day) in week_days.iter().enumerate() {
                        div {
                            class: format!("p-2 text-xs font-medium text-center border-r last:border-r-0 {}",
                                if i == 0 || i == 6 || past_days[i] { 
                                    "bg-gray-50 text-gray-500" 
                                } else { 
                                    "text-gray-700 bg-gray-100" 
                                }
                            ),
                            div { 
                                "{day}"
                            }
                            div { 
                                class: "text-xs text-gray-500 mt-1",
                                "{week_dates[i]}"
                            }
                        }
                    }
                }
            }
            
            // Calendar body - 5-minute resolution grid
            div {
                class: "grid grid-cols-8 gap-0 pt-4 mb-4 relative border-b-2 border-gray-600", // Add relative positioning and bottom border
                
                // Top horizontal line across all columns
                for _col in 0..8 {
                    div {
                        class: "border-t-2 border-gray-600 h-px",
                    }
                }
                
                // Create rows for each 5-minute slot
                for slot_idx in 0..total_slots {
                    {
                        let hour = start_hour + (slot_idx / 12);
                        let minute = (slot_idx % 12) * 5;
                        let is_hour_boundary = minute == 0;
                        
                        rsx! {
                            // Empty time column (no text, just spacing)
                            div {
                                class: format!("h-1 border-r bg-white {}",
                                    if is_hour_boundary {
                                        "border-t-2 border-gray-600"
                                    } else {
                                        ""
                                    }
                                ),
                            }                            // Day columns
                            for day_idx in 0..7 {
                                {
                                    let is_current_time = is_current_time_slot(day_idx, hour, minute, current_week_offset, &timezone);
                                    
                                    rsx! {
                                        div {
                                            class: format!("h-1 border-r relative group {}",
                                                if is_hour_boundary {
                                                    format!("border-t-2 border-gray-600 {}", 
                                                        if is_current_time {
                                                            "bg-blue-300 hover:bg-blue-400 cursor-pointer"
                                                        } else if is_time_slot_unavailable_optimized(&events, day_idx, hour, minute, &target_dates, &past_days) {
                                                            "bg-gray-300 hover:bg-gray-400 cursor-not-allowed"
                                                        } else if is_time_slot_busy_optimized(&events, day_idx, hour, minute, &target_dates) {
                                                            "bg-gray-400 hover:bg-gray-500 cursor-not-allowed"
                                                        } else {
                                                            "bg-green-200 hover:bg-green-300 cursor-pointer"
                                                        }
                                                    )
                                                } else {
                                                    if is_current_time {
                                                        "bg-blue-300 hover:bg-blue-400 cursor-pointer".to_string()
                                                    } else if is_time_slot_unavailable_optimized(&events, day_idx, hour, minute, &target_dates, &past_days) {
                                                        "bg-gray-300 hover:bg-gray-400 cursor-not-allowed".to_string()
                                                    } else if is_time_slot_busy_optimized(&events, day_idx, hour, minute, &target_dates) {
                                                        "bg-gray-400 hover:bg-gray-500 cursor-not-allowed".to_string()
                                                    } else {
                                                        "bg-green-200 hover:bg-green-300 cursor-pointer".to_string()
                                                    }
                                                }
                                            ),
                                            title: format!("{}:{:02}{} - {}:{:02}{}",
                                                if hour == 0 { 12 } else if hour > 12 { hour - 12 } else { hour },
                                                minute,
                                                if hour < 12 { "AM" } else { "PM" },
                                                if hour == 0 && minute >= 55 { 12 } else if hour >= 12 && minute >= 55 { if hour == 12 { 1 } else { hour - 11 } } else if minute >= 55 { hour + 1 } else { if hour == 0 { 12 } else if hour > 12 { hour - 12 } else { hour } },
                                                if minute >= 55 { 0 } else { minute + 5 },
                                                if hour < 12 && (hour != 11 || minute < 55) { "AM" } else if hour >= 12 && (hour != 23 || minute < 55) { "PM" } else if hour == 11 && minute >= 55 { "PM" } else { "AM" }
                                            ),
                                            
                                            // Tooltip
                                            div {
                                                class: "absolute bottom-full left-1/2 transform -translate-x-1/2 mb-1 px-2 py-1 bg-black text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity duration-200 pointer-events-none z-50 whitespace-nowrap",
                                                {
                                                    let day_name = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"][day_idx as usize];
                                                    let target_date = &target_dates[day_idx as usize];
                                                    let date_str = if target_date.len() >= 8 {
                                                        let month = &target_date[4..6];
                                                        let day = &target_date[6..8];
                                                        format!("{}/{}", month, day)
                                                    } else {
                                                        "??/??".to_string()
                                                    };
                                                    let tooltip_text = format!("{} {} {}:{:02}{} - {}:{:02}{}",
                                                        day_name,
                                                        date_str,
                                                        if hour == 0 { 12 } else if hour > 12 { hour - 12 } else { hour },
                                                        minute,
                                                        if hour < 12 { "AM" } else { "PM" },
                                                        if hour == 0 && minute >= 55 { 12 } else if hour >= 12 && minute >= 55 { if hour == 12 { 1 } else { hour - 11 } } else if minute >= 55 { hour + 1 } else { if hour == 0 { 12 } else if hour > 12 { hour - 12 } else { hour } },
                                                        if minute >= 55 { 0 } else { minute + 5 },
                                                        if hour < 12 && (hour != 11 || minute < 55) { "AM" } else if hour >= 12 && (hour != 23 || minute < 55) { "PM" } else if hour == 11 && minute >= 55 { "PM" } else { "AM" }
                                                    );
                                                    if is_current_time {
                                                        format!("{} (Current Time)", tooltip_text)
                                                    } else {
                                                        tooltip_text
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Overlapping time labels layer
            div {
                class: "absolute left-0 top-[72px] pointer-events-none", // Position after header (header height ~72px)
                
                // Time labels positioned over the grid
                for hour in start_hour..end_hour {
                    {
                        let row_offset = (hour - start_hour) * 12; // 12 slots per hour
                        let top_position = row_offset * 4; // 4px per cell (h-1)
                        
                        rsx! {
                            div {
                                class: "absolute text-xs text-gray-700 font-medium flex items-start justify-center px-1",
                                style: format!("top: {}px; width: auto; height: 24px; padding-top: 2px;", top_position),
                                {
                                    if hour <= 12 { 
                                        if hour == 0 { 
                                            "12AM".to_string()
                                        } else {
                                            format!("{}AM", hour)
                                        }
                                    } else { 
                                        format!("{}PM", hour - 12)
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // Legend
            div {
                class: "bg-gray-50 px-4 py-2 border-t text-xs",
                div {
                    class: "flex gap-6",
                    div {
                        class: "flex items-center gap-1",
                        div { class: "w-3 h-3 bg-green-200 border border-green-400 rounded" }
                        span { "Available" }
                    }
                    div {
                        class: "flex items-center gap-1",
                        div { class: "w-3 h-3 bg-gray-300 border border-gray-500 rounded" }
                        span { "Unavailable" }
                    }
                }
            }
        }
    }
}

fn is_time_slot_busy_optimized(events: &[CalendarEvent], day_idx: i32, hour: i32, minute: i32, target_dates: &[String]) -> bool {
    let target_date = &target_dates[day_idx as usize];
    
    // Convert the target time slot to minutes since midnight for easier comparison
    let target_start_minutes = hour * 60 + minute;
    let target_end_minutes = target_start_minutes + 5; // 5-minute slot
    
    // Check if any event overlaps with this 5-minute time slot
    for event in events {
        if &event.start_date == target_date {
            let event_start_minutes = event.start_hour * 60 + event.start_minute;
            let event_end_minutes = event.end_hour * 60 + event.end_minute;
            
            // Check for overlap: event starts before slot ends AND event ends after slot starts
            if event_start_minutes < target_end_minutes && event_end_minutes > target_start_minutes {
                return true;
            }
        }
    }
    
    false
}

// Optimized unavailable function using pre-calculated data
fn is_time_slot_unavailable_optimized(events: &[CalendarEvent], day_idx: i32, hour: i32, minute: i32, target_dates: &[String], past_days: &[bool]) -> bool {
    // Weekend check (0=Sunday, 6=Saturday)
    if day_idx == 0 || day_idx == 6 {
        return true;
    }
    
    // Past date check using pre-calculated data
    if past_days[day_idx as usize] {
        return true;
    }
    
    // Busy from calendar events using optimized function
    is_time_slot_busy_optimized(events, day_idx, hour, minute, target_dates)
}

// Check if a time slot corresponds to the current time (timezone-aware)
fn is_current_time_slot(day_idx: i32, hour: i32, minute: i32, current_week_offset: i32, timezone: &str) -> bool {
    // Get current time in the specified timezone
    let js_code = format!(
        r#"(() => {{
            const now = new Date();
            const formatter = new Intl.DateTimeFormat('en-CA', {{
                timeZone: '{}',
                year: 'numeric',
                month: '2-digit',
                day: '2-digit',
                hour: '2-digit',
                minute: '2-digit',
                weekday: 'short',
                hour12: false
            }});
            const parts = formatter.formatToParts(now);
            const result = {{}};
            parts.forEach(part => result[part.type] = part.value);
            return JSON.stringify(result);
        }})()"#,
        timezone
    );
    
    if let Ok(result) = js_sys::eval(&js_code) {
        if let Some(json_str) = result.as_string() {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json_str) {
                let current_hour = parsed["hour"].as_str().unwrap_or("00").parse::<i32>().unwrap_or(0);
                let current_minute = parsed["minute"].as_str().unwrap_or("00").parse::<i32>().unwrap_or(0);
                let current_weekday = parsed["weekday"].as_str().unwrap_or("Mon");
                
                // Convert weekday to day_idx (0=Sunday, 6=Saturday)
                let current_day_idx = match current_weekday {
                    "Sun" => 0,
                    "Mon" => 1,
                    "Tue" => 2,
                    "Wed" => 3,
                    "Thu" => 4,
                    "Fri" => 5,
                    "Sat" => 6,
                    _ => 0,
                };
                
                // Check if we're in the current week
                if current_week_offset != 0 {
                    return false;
                }
                
                // Check if it's the current day
                if day_idx != current_day_idx {
                    return false;
                }
                
                // Check if it's the current hour and minute range (within 5-minute slot)
                if hour == current_hour {
                    let slot_start = minute;
                    let slot_end = minute + 5;
                    return current_minute >= slot_start && current_minute < slot_end;
                }
                
                return false;
            }
        }
    }
    
    // Fallback to browser local time
    let now = js_sys::Date::new_0();
    let current_hour = now.get_hours() as i32;
    let current_minute = now.get_minutes() as i32;
    let current_day = now.get_day() as i32; // 0 = Sunday, 1 = Monday, etc.
    
    // Convert Sunday-based (0-6) to Sunday-based (0-6 where 0 = Sunday)
    let current_day_sunday_based = current_day;
    
    // Check if we're in the current week
    if current_week_offset != 0 {
        return false;
    }
    
    // Check if it's the current day
    if day_idx != current_day_sunday_based {
        return false;
    }
    
    // Check if it's the current hour and minute range (within 5-minute slot)
    if hour == current_hour {
        let slot_start = minute;
        let slot_end = minute + 5;
        return current_minute >= slot_start && current_minute < slot_end;
    }
    
    false
}

fn is_in_past_with_timezone(day_idx: i32, week_offset: i32, timezone: &str) -> bool {
    // Get today's date in the specified timezone
    let js_code = format!(
        r#"(() => {{
            const now = new Date();
            const formatter = new Intl.DateTimeFormat('en-CA', {{
                timeZone: '{}',
                year: 'numeric',
                month: '2-digit',
                day: '2-digit'
            }});
            const parts = formatter.formatToParts(now);
            const result = {{}};
            parts.forEach(part => result[part.type] = part.value);
            return result.year + result.month + result.day;
        }})()"#,
        timezone
    );
    
    let today_str = if let Ok(result) = js_sys::eval(&js_code) {
        result.as_string().unwrap_or_else(|| {
            // Fallback to browser local time
            let today = js_sys::Date::new_0();
            format!("{:04}{:02}{:02}", 
                today.get_full_year() as i32,
                (today.get_month() as i32) + 1,
                today.get_date() as i32
            )
        })
    } else {
        // Fallback to browser local time
        let today = js_sys::Date::new_0();
        format!("{:04}{:02}{:02}", 
            today.get_full_year() as i32,
            (today.get_month() as i32) + 1,
            today.get_date() as i32
        )
    };
    
    // Get the target date (also timezone-aware)
    let target_date_str = get_date_for_day_with_timezone(day_idx, week_offset, timezone);
    
    // Compare dates as strings (YYYYMMDD format allows string comparison)
    target_date_str < today_str
}

fn get_date_for_day_with_timezone(day_idx: i32, week_offset: i32, timezone: &str) -> String {
    // Get current date in the specified timezone
    let js_code = format!(
        r#"(() => {{
            const now = new Date();
            const formatter = new Intl.DateTimeFormat('en-CA', {{
                timeZone: '{}',
                year: 'numeric',
                month: '2-digit',
                day: '2-digit',
                weekday: 'short'
            }});
            const parts = formatter.formatToParts(now);
            const result = {{}};
            parts.forEach(part => result[part.type] = part.value);
            
            // Get current day index (0=Sunday, 6=Saturday)
            const currentDayName = result.weekday;
            const dayMapping = {{'Sun': 0, 'Mon': 1, 'Tue': 2, 'Wed': 3, 'Thu': 4, 'Fri': 5, 'Sat': 6}};
            const currentDayIdx = dayMapping[currentDayName] || 0;
            
            // Calculate offset to Sunday (start of week)
            const daysToSunday = -currentDayIdx;
            const totalDayOffset = daysToSunday + ({} * 7) + {};
            
            // Create target date
            const currentDate = parseInt(result.day);
            const targetDate = new Date(now);
            targetDate.setDate(currentDate + totalDayOffset);
            
            // Format in the target timezone
            const targetFormatter = new Intl.DateTimeFormat('en-CA', {{
                timeZone: '{}',
                year: 'numeric',
                month: '2-digit',
                day: '2-digit'
            }});
            const targetParts = targetFormatter.formatToParts(targetDate);
            const targetResult = {{}};
            targetParts.forEach(part => targetResult[part.type] = part.value);
            
            return targetResult.year + targetResult.month + targetResult.day;
        }})()"#,
        timezone, week_offset, day_idx, timezone
    );
    
    if let Ok(result) = js_sys::eval(&js_code) {
        if let Some(date_str) = result.as_string() {
            return date_str;
        }
    }
    
    // Fallback to old implementation
    get_date_for_day(day_idx, week_offset)
}

fn get_date_for_day(day_idx: i32, week_offset: i32) -> String {
    // Get current date using JavaScript Date API (timezone-aware)
    let date = js_sys::Date::new_0();
    
    // Get current date components (already in browser's local timezone)
    let current_date = date.get_date() as i32;
    let current_day_of_week = date.get_day() as i32; // 0 = Sunday, 1 = Monday, etc.
    
    // Convert to Sunday-based indexing (0 = Sunday, 6 = Saturday)
    let current_day_of_week = current_day_of_week; // Already 0=Sunday in JavaScript
    
    // Calculate days from today to the start of current week (Sunday)
    let days_to_sunday = -current_day_of_week;
    
    // Calculate total offset in days
    let total_day_offset = days_to_sunday + (week_offset * 7) + day_idx;
    
    // Create a new date by adding the offset (timezone-aware)
    let target_date = js_sys::Date::new_0();
    target_date.set_date((current_date + total_day_offset) as u32);
    
    // Format as YYYYMMDD (local timezone)
    let year = target_date.get_full_year() as i32;
    let month = (target_date.get_month() as i32) + 1; // Convert to 1-based
    let day = target_date.get_date() as i32;
    
    format!("{:04}{:02}{:02}", year, month, day)
}

#[component]
pub fn Cal() -> Element {
    let mut availability_data = use_signal(|| Vec::<CalendarEvent>::new());
    let mut loading = use_signal(|| true);
    let mut error_msg = use_signal(|| None::<String>);
    let mut current_week_offset = use_signal(|| 0i32);
    let mut selected_timezone = use_signal(|| get_browser_timezone()); // Start with browser timezone
    
    // Configuration variables
    let use_timezone_dropdown = false; // Set to true to show timezone dropdown, false to show browser timezone as text
    let max_weeks_ahead = 5; // Maximum weeks user can navigate ahead (0 = current week only, 5 = up to 5 weeks ahead)
    let weeks_ahead = 4;
    let start_hour = 8;  // 8 AM
    let end_hour = 18;   // 6 PM
    
    // Get browser timezone and create dynamic timezone options
    let browser_tz = get_browser_timezone();
    
    // Create timezone options with owned strings to handle browser timezone
    let mut timezone_options: Vec<(String, String)> = vec![
        ("America/Los_Angeles".to_string(), "Pacific Time (PT)".to_string()),
        ("America/New_York".to_string(), "Eastern Time (ET)".to_string()),
        ("Europe/Rome".to_string(), "Central European Time (CET)".to_string()),
        ("UTC".to_string(), "Coordinated Universal Time (UTC)".to_string()),
    ];
    
    // Add browser timezone if it's not already in the list
    let browser_tz_exists = timezone_options.iter().any(|(tz_id, _)| *tz_id == browser_tz);
    if !browser_tz_exists {
        // Get a user-friendly name for the browser timezone
        let browser_tz_display = get_timezone_display(&browser_tz);
        timezone_options.insert(0, (browser_tz.clone(), browser_tz_display)); // Add at the beginning
    }
    
    // Multiple CORS proxy options (try them in order if one fails)
    let calendar_urls = vec![
        "https://corsproxy.io/?https://outlook.office365.com/owa/calendar/88f1cc88162748f694d9f22f5dc7b664@uga.edu/7de187184ca64395b68c8b830208b9a03053105374142465705/calendar.ics",
        // Backup proxies if the first one fails:
        // "https://api.codetabs.com/v1/proxy?quest=https://outlook.office365.com/owa/calendar/88f1cc88162748f694d9f22f5dc7b664@uga.edu/7de187184ca64395b68c8b830208b9a03053105374142465705/calendar.ics",
        // "https://thingproxy.freeboard.io/fetch/https://outlook.office365.com/owa/calendar/88f1cc88162748f694d9f22f5dc7b664@uga.edu/7de187184ca64395b68c8b830208b9a03053105374142465705/calendar.ics",
    ];

    // Fetch calendar data on component mount and when timezone changes
    use_effect({
        let selected_timezone = selected_timezone.clone();
        move || {
            let urls = calendar_urls.clone();
            let current_timezone = selected_timezone();
            spawn_local(async move {
                let url_refs: Vec<&str> = urls.iter().map(|s| &**s).collect();
                match fetch_calendar_data(url_refs, max_weeks_ahead).await {
                    Ok(mut events) => {
                        // Re-parse events with the selected timezone
                        for event in &mut events {
                            let (start_date, start_hour, start_minute) = parse_ics_datetime_with_timezone(&event.start, &current_timezone);
                            let (_, end_hour, end_minute) = parse_ics_datetime_with_timezone(&event.end, &current_timezone);
                            
                            event.start_date = start_date;
                            event.start_hour = start_hour;
                            event.start_minute = start_minute;
                            event.end_hour = end_hour;
                            event.end_minute = end_minute;
                        }
                        
                        // Debug: log all final events (only in debug mode)
                        #[cfg(debug_assertions)]
                        {
                            // web_sys::console::log_1(&format!("=== FINAL EVENTS ({}) ===", events.len()).into());
                            // for (i, event) in events.iter().enumerate() {
                            //     web_sys::console::log_1(&format!("Event {}: {} ({}:{:02}-{}:{:02}) on {}", 
                            //         i, event.start, event.start_hour, event.start_minute, event.end_hour, event.end_minute, event.start_date).into());
                            // }
                            // web_sys::console::log_1(&"=== END EVENTS ===".into());
                        }
                        
                        availability_data.set(events);
                        loading.set(false);
                    }
                    Err(e) => {
                        error_msg.set(Some(format!("Failed to load calendar: {}", e)));
                        loading.set(false);
                    }
                }
            });
        }
    });

    rsx! {
        div {
            class: format!("{} relative z-10", CSS_CONTENT_CONTAINER),
            
            div {
                class: CSS_CONTENT_CARD,
                
                div {
                    class: "mb-8",
                    h1 {
                        class: CSS_PAGE_TITLE,
                        "Schedule a Meeting"
                    }

                    div {
                        class: "flex items-center mb-4",
                        
                        if use_timezone_dropdown {
                            div {
                                class: "flex items-center flex-shrink min-w-0",
                                span {
                                    class: "text-gray-600 text-lg mr-2",
                                    "Timezone:"
                                }
                                select {
                                    class: "flex-shrink min-w-0 max-w-full w-32 sm:w-auto px-3 py-1 border border-gray-300 rounded-md text-sm bg-white text-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 overflow-hidden text-ellipsis",
                                    style: "width: auto; max-width: 100%;",
                                    value: "{selected_timezone()}",
                                    onchange: move |evt| {
                                        selected_timezone.set(evt.value());
                                        // Trigger calendar data refresh when timezone changes
                                        loading.set(true);
                                    },
                                    
                                    for (tz_id, tz_display) in timezone_options.iter() {
                                        option {
                                            value: "{tz_id}",
                                            selected: selected_timezone() == *tz_id,
                                            "{tz_display}"
                                        }
                                    }
                                }
                            }
                        } else {
                            span {
                                class: "text-gray-600 text-lg",
                                "Timezone: {get_timezone_display(&selected_timezone())}"
                            }
                        }
                    }

                    p {
                        class: "text-gray-600 text-lg mb-6",
                        "This calendar is designed to simplify scheduling meetings with my collaborators and students. 
                        The calendar now supports recurring events (daily, weekly, monthly, yearly) and automatically 
                        generates all instances based on recurrence rules. Please note that this represents only a partial 
                        view of my calendar and may not be 100% accurate. However, I hope this visualization will help you 
                        identify potential meeting times and streamline our scheduling process. Please reach out to me 
                        directly to confirm and finalize any meeting arrangements."
                    }

                    // Week navigation
                    div {
                        class: "flex items-center justify-between mb-4 p-4 bg-gray-50 rounded-lg",
                        
                        button {
                            class: format!("flex items-center gap-2 px-3 py-2 rounded transition-colors {}",
                                if current_week_offset() <= 0 {
                                    "bg-gray-300 text-gray-500 cursor-not-allowed"
                                } else {
                                    "bg-red-600 text-white hover:bg-red-700 cursor-pointer"
                                }
                            ),
                            disabled: current_week_offset() <= 0,
                            onclick: move |_| {
                                if current_week_offset() > 0 {
                                    current_week_offset.set(current_week_offset() - 1);
                                }
                            },
                            "←"
                        }
                        
                        div {
                            class: "text-lg font-medium text-gray-700",
                            if current_week_offset() == 0 {
                                "This Week"
                            } else if current_week_offset() == 1 {
                                "Next Week"
                            } else if current_week_offset() > 1 {
                                "{current_week_offset()} weeks ahead"
                            } else {
                                "{-current_week_offset()} weeks ago"
                            }
                        }
                        
                        button {
                            class: format!("flex items-center gap-2 px-3 py-2 rounded transition-colors {}",
                                if current_week_offset() >= max_weeks_ahead {
                                    "bg-gray-300 text-gray-500 cursor-not-allowed"
                                } else {
                                    "bg-red-600 text-white hover:bg-red-700 cursor-pointer"
                                }
                            ),
                            disabled: current_week_offset() >= max_weeks_ahead,
                            onclick: move |_| {
                                if current_week_offset() < max_weeks_ahead {
                                    current_week_offset.set(current_week_offset() + 1);
                                }
                            },
                            "→"
                        }
                    }

                    if loading() {
                        div {
                            class: "text-center py-8",
                            "Loading calendar..."
                        }
                    } else if let Some(error) = error_msg() {
                        div {
                            class: "text-red-600 text-center py-8",
                            "{error}"
                        }
                    } else {
                        CalendarGrid {
                            events: availability_data(),
                            weeks_ahead: weeks_ahead,
                            timezone: selected_timezone().to_string(),
                            current_week_offset: current_week_offset(),
                            start_hour: start_hour,
                            end_hour: end_hour
                        }
                    }
                }
            }
        }
    }
}
