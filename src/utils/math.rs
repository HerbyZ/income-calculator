pub fn round(number: f64) -> Result<f64, String> {
    let num_str = number.to_string();
    let num_parts: Vec<&str> = num_str.split(".").collect();

    let parts_amount = num_parts.len();
    if parts_amount == 0 || parts_amount > 2 {
        return Err(format!(
            "Given value as number {} cannot be rounded. Check that number is correct.",
            number
        ));
    }

    if parts_amount == 1 {
        return Ok(number);
    }

    let int_part = num_parts[0];
    let fractional_part = num_parts[1];

    if int_part.len() >= 4 {
        return Ok(round::round(number, 0));
    }

    if int_part.len() >= 1 {
        return Ok(round::round(number, 4));
    }

    let mut zeros_count = 0;
    for digit_char in fractional_part.split("") {
        if digit_char == "0" {
            zeros_count += 1;
        } else {
            break;
        }
    }

    Ok(round::round(number, zeros_count + 2))
}
