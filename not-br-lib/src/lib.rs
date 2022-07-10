pub mod not_br {
    extern crate unicode_segmentation;

    use std::ffi::{CStr, CString};
    use std::ops::Add;
    use unicode_segmentation::{UnicodeSegmentation};


    pub enum InputType {
        HTML,
        Markdown,
    }

    impl InputType {
        pub fn get_open(&self) -> &str {
            match self {
                InputType::HTML => { "<b>" }
                InputType::Markdown => { "**" }
            }
        }
        pub fn get_closing(&self) -> &str {
            match self {
                InputType::HTML => { "</b>" }
                InputType::Markdown => { "**" }
            }
        }
    }


    pub trait NotBrProcess<T> {
        fn process_text(self, fixation: u64, bold_percentage: f64, input_type: InputType) -> Result<T, ()>;
    }

    impl NotBrProcess<String> for &str {
        fn process_text(self, frequency: u64, bold_percentage: f64, input_type: InputType) -> Result<String, ()> {
            if !(0_f64 <= bold_percentage && bold_percentage <= 1_f64) { return Err(()); }
            let input = self;
            let words = UnicodeSegmentation::split_word_bounds(input).collect::<Vec<&str>>();
            let retval = words.into_iter().enumerate().fold(String::new(), |mut acc, (i, w)| {
                if i % frequency as usize != 0 || UnicodeSegmentation::unicode_words(w).collect::<Vec<&str>>().len() == 0 { return acc.add(w); } else {
                    let graphemes = UnicodeSegmentation::graphemes(w, true)
                        .collect::<Vec<&str>>();
                    let word_len = graphemes.len();
                    graphemes.into_iter().enumerate()
                        .for_each(|(j, graph)| {
                            if j == 0 && ((word_len as f64 * bold_percentage).ceil() as isize) > 0 { acc.push_str(input_type.get_open()) }
                            acc.push_str(graph);
                            if j as isize == ((word_len as f64 * bold_percentage).ceil() as isize) - 1 { acc.push_str(input_type.get_closing()) }
                        })
                }
                acc
            });
            Ok(retval)
        }
    }

    impl NotBrProcess<CString> for &CStr {
        fn process_text(self, frequency: u64, bold_percentage: f64, input_type: InputType) -> Result<CString, ()> {
            if !(0_f64 <= bold_percentage && bold_percentage <= 1_f64) { return Err(()); }
            let input = match self.to_str() {
                Ok(s) => { s }
                Err(_) => { return Err(()); }
            };
            let retval = input.process_text(frequency, bold_percentage, input_type);
            match retval {
                Ok(retval) => {
                    Ok(CString::new(retval).unwrap())
                }
                Err(_) => { Err(()) }
            }
        }
    }
}


#[cfg(test)]
mod not_br_tests {
    #[cfg(test)]
    mod test_process_text_str {
        use crate::not_br::{InputType, NotBrProcess};

        #[test]
        fn test_markdown_simple_ascii_only_0ptct() {
            let input = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr.";

            let expected_html = String::from(input);
            let expected_markdown = String::from(input);

            let output_markdown = input.process_text(1, 0., InputType::Markdown);
            let output_html = input.process_text(1, 0., InputType::HTML);

            assert_eq!(output_markdown.unwrap(), expected_markdown);
            assert_eq!(output_html.unwrap(), expected_html);
        }

        #[test]
        fn test_markdown_simple_ascii_only_50pct() {
            let input = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr.";

            let output_markdown = input.process_text(1, 0.5, InputType::Markdown);
            let output_html = input.process_text(1, 0.5, InputType::HTML);

            let expected_markdown = String::from("**Lor**em **ips**um **dol**or **si**t **am**et, **conse**tetur **sadip**scing **eli**tr.");
            let expected_html = String::from("<b>Lor</b>em <b>ips</b>um <b>dol</b>or <b>si</b>t <b>am</b>et, <b>conse</b>tetur <b>sadip</b>scing <b>eli</b>tr.");

            assert_eq!(output_html.unwrap(), expected_html);
            assert_eq!(output_markdown.unwrap(), expected_markdown);
        }


        #[test]
        fn test_markdown_simple_ascii_only_100pct() {
            let input = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr.";

            let output_markdown = input.process_text(1, 1., InputType::Markdown);
            let output_html = input.process_text(1, 1., InputType::HTML);

            let expected_markdown = String::from("**Lorem** **ipsum** **dolor** **sit** **amet**, **consetetur** **sadipscing** **elitr**.");
            let expected_html = String::from("<b>Lorem</b> <b>ipsum</b> <b>dolor</b> <b>sit</b> <b>amet</b>, <b>consetetur</b> <b>sadipscing</b> <b>elitr</b>.");

            assert_eq!(output_markdown.unwrap(), expected_markdown);
            assert_eq!(output_html.unwrap(), expected_html);
        }

        #[test]
        fn test_markdown_simple_panagram_german_0pct() {
            let input = "Büß du ja zwölf Qirsch, Kämpe!";

            let expected_html = String::from(input);
            let expected_markdown = String::from(input);

            let output_markdown = input.process_text(1, 0., InputType::Markdown);
            let output_html = input.process_text(1, 0., InputType::HTML);

            assert_eq!(output_markdown.unwrap(), expected_markdown);
            assert_eq!(output_html.unwrap(), expected_html);
        }

        #[test]
        fn test_markdown_simple_panagram_german_50pct() {
            let input = "Büß du ja zwölf Qirsch, Kämpe!";

            let output_markdown = input.process_text(1, 0.5, InputType::Markdown);
            let output_html = input.process_text(1, 0.5, InputType::HTML);

            let expected_markdown = String::from("**Bü**ß **d**u **j**a **zwö**lf **Qir**sch, **Käm**pe!");
            let expected_html = String::from("<b>Bü</b>ß <b>d</b>u <b>j</b>a <b>zwö</b>lf <b>Qir</b>sch, <b>Käm</b>pe!");

            assert_eq!(output_html.unwrap(), expected_html);
            assert_eq!(output_markdown.unwrap(), expected_markdown);
        }


        #[test]
        fn test_markdown_simple_panagram_german_100pct() {
            let input = "Büß du ja zwölf Qirsch, Kämpe!";

            let output_markdown = input.process_text(1, 1., InputType::Markdown);
            let output_html = input.process_text(1, 1., InputType::HTML);

            let expected_markdown = String::from("**Büß** **du** **ja** **zwölf** **Qirsch**, **Kämpe**!");
            let expected_html = String::from("<b>Büß</b> <b>du</b> <b>ja</b> <b>zwölf</b> <b>Qirsch</b>, <b>Kämpe</b>!");

            assert_eq!(output_markdown.unwrap(), expected_markdown);
            assert_eq!(output_html.unwrap(), expected_html);
        }
    }
}
