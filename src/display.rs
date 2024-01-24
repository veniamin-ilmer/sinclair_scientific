use chips::tms0800;
use chips::shifter;

pub(super) struct Display {
  display: web_sys::Element,
  current_str: String,
}

impl Display {
  pub(super) fn new() -> Self {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let display = document.get_element_by_id("display").unwrap();
    
    Self {
      display,
      current_str: String::from("         "),
    }
  }

  pub fn run_refresh_cycle(&mut self, chip: &tms0800::TMS0800) {
    let mut buffer = Vec::with_capacity(11);
    let mut exp_buffer = Vec::with_capacity(3);
    let mut a = chip.alu.a.clone();
    let direction = shifter::Direction::Left;
    //First is the mantissa sign
    let digit = a.read_nibble(direction);
    buffer.push(if digit.value() == 5 { '-' } else { ' ' });
    a.shift_with_nibble(direction, digit);
    //Second is the exp sign
    let digit = a.read_nibble(direction);
    exp_buffer.push(if digit.value() == 5 { '-' } else { ' ' });
    a.shift_with_nibble(direction, digit);
    //Exponent
    for _ in 0..2 {
      let digit = a.read_nibble(direction);
      exp_buffer.push((digit.value() + 48) as char);
      a.shift_with_nibble(direction, digit);
    }
    //Mantissa
    for location in 0..5 {
      let digit = a.read_nibble(direction);
      buffer.push((digit.value() + 48) as char);
      if location == 0 {
        buffer.push('.');
      }
      a.shift_with_nibble(direction, digit);
    }
    buffer.append(&mut exp_buffer);
    
    let new_str = buffer.iter().collect::<String>();
    if self.current_str != new_str {
      self.display.set_text_content(Some(&new_str));
      self.current_str = new_str.clone();
    }
  }
}