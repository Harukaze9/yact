pub enum SpecialOperationType {
    Add,
    Remove,
    Peak,
    // Copy,
}

impl SpecialOperationType {
    fn from_str(op: &str) -> Option<Self> {
        match op {
            "-a" => Some(Self::Add),
            "-r" => Some(Self::Remove),
            "-p" => Some(Self::Peak),
            _ => None,
        }
    }
}


pub fn detect_special_operation<'a>(params: &'a [&'a str]) -> Result<Option<(SpecialOperationType, &'a [&'a str], &'a [&'a str])>, &'static str> {
    let mut found_operation: Option<SpecialOperationType> = None;
    let mut found_idx: Option<usize> = None;

    for (idx, &param) in params.iter().enumerate() {
        if let Some(operation) = SpecialOperationType::from_str(param) {
            if found_idx.is_some() {
                // Error: More than one operation detected
                return Err("Multiple special operations detected.");
            }
            found_operation = Some(operation);
            found_idx = Some(idx);
        }
    }

    if let (Some(operation), Some(idx)) = (found_operation, found_idx) {
        let path = &params[0..idx];
        let args = &params[idx+1..];
        Ok(Some((operation, path, args)))
    } else {
        Ok(None)
    }
}