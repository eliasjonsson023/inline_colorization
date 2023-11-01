use inline_colorization::*;

fn main() {
  println!("no_style");

  // styles
  println!("{style_bold}style_bold{style_reset}");
  println!("{style_underline}style_underline{style_reset}");

  // colors
  println!("{bg_white}{color_black}color_black{color_reset}{bg_reset}");
  println!("{color_red}color_red{color_reset}");
  println!("{color_green}color_green{color_reset}");
  println!("{color_yellow}color_yellow{color_reset}");
  println!("{color_blue}color_blue{color_reset}");
  println!("{color_magenta}color_magenta{color_reset}");
  println!("{color_cyan}color_cyan{color_reset}");
  println!("{color_white}color_white{color_reset}");
  println!("{bg_white}{color_bright_black}color_bright_black{color_reset}{bg_reset}");
  println!("{color_bright_red}color_bright_red{color_reset}");
  println!("{color_bright_green}color_bright_green{color_reset}");
  println!("{color_bright_yellow}color_bright_yellow{color_reset}");
  println!("{color_bright_blue}color_bright_blue{color_reset}");
  println!("{color_bright_magenta}color_bright_magenta{color_reset}");
  println!("{color_bright_cyan}color_bright_cyan{color_reset}");
  println!("{color_bright_white}color_bright_white{color_reset}");

  // backgrounds
  println!("{bg_black}bg_black{bg_reset}");
  println!("{bg_red}bg_red{bg_reset}");
  println!("{bg_green}bg_green{bg_reset}");
  println!("{bg_yellow}bg_yellow{bg_reset}");
  println!("{bg_blue}bg_blue{bg_reset}");
  println!("{bg_magenta}bg_magenta{bg_reset}");
  println!("{bg_cyan}bg_cyan{bg_reset}");
  println!("{color_black}{bg_white}bg_white{bg_reset}{color_reset}");
  println!("{bg_bright_black}bg_bright_black{bg_reset}");
  println!("{bg_bright_red}bg_bright_red{bg_reset}");
  println!("{bg_bright_green}bg_bright_green{bg_reset}");
  println!("{bg_bright_yellow}bg_bright_yellow{bg_reset}");
  println!("{bg_bright_blue}bg_bright_blue{bg_reset}");
  println!("{bg_bright_magenta}bg_bright_magenta{bg_reset}");
  println!("{bg_bright_cyan}bg_bright_cyan{bg_reset}");
  println!("{color_black}{bg_bright_white}bg_bright_white{bg_reset}{color_reset}");
}
