pub mod not_br {
    extern crate unicode_segmentation;

    use std::ffi::{CStr, CString};
    use std::ops::Add;
    use std::str::FromStr;
    use unicode_segmentation::{UnicodeSegmentation};


    #[derive(Clone, Copy)]
    pub enum OutputType {
        HTML,
        Markdown,
    }

    impl OutputType {
        pub fn get_open(&self) -> &str {
            match self {
                OutputType::HTML => { "<b>" }
                OutputType::Markdown => { "**" }
            }
        }
        pub fn get_closing(&self) -> &str {
            match self {
                OutputType::HTML => { "</b>" }
                OutputType::Markdown => { "**" }
            }
        }
    }

    impl FromStr for OutputType {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "HTML" | "html" => Ok(OutputType::HTML),
                "Markdown" | "markdown" | "md" => Ok(OutputType::Markdown),
                _ => { Err(()) }
            }
        }
    }


    pub trait NotBrProcess<T> {
        fn process_text(self, fixation: u64, bold_percentage: f64, input_type: OutputType) -> Result<T, ()>;
    }

    impl NotBrProcess<String> for &str {
        fn process_text(self, frequency: u64, bold_percentage: f64, input_type: OutputType) -> Result<String, ()> {
            if frequency == 0 { return Ok(String::from(self)); }
            if !(0_f64 <= bold_percentage && bold_percentage <= 1_f64) { return Err(()); }
            let input = self;
            let words = UnicodeSegmentation::split_word_bounds(input).collect::<Vec<&str>>();
            let mut word_counter: usize = 0;
            let retval = words.into_iter().fold(String::new(), |mut acc, w| {
                let is_word = UnicodeSegmentation::unicode_words(w).collect::<Vec<&str>>().len() != 0;
                if word_counter % frequency as usize != 0 || !is_word {
                    if is_word { word_counter += 1; };
                    return acc.add(w);
                } else {
                    if is_word { word_counter += 1; };
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
        fn process_text(self, frequency: u64, bold_percentage: f64, input_type: OutputType) -> Result<CString, ()> {
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

    pub fn process<I: NotBrProcess<O>, O>(input: I, frequency: u64, bold_percentage: f64, input_type: OutputType) -> Result<O, ()> {
        input.process_text(frequency, bold_percentage, input_type)
    }
}


#[cfg(test)]
mod not_br_tests {
    #[cfg(test)]
    mod test_process_text_str {
        use crate::not_br::{OutputType, NotBrProcess};

        #[test]
        fn test_markdown_simple_ascii_only_0ptct() {
            let input = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr.";

            let expected_html = String::from(input);
            let expected_markdown = String::from(input);

            let output_markdown = input.process_text(1, 0., OutputType::Markdown);
            let output_html = input.process_text(1, 0., OutputType::HTML);

            assert_eq!(output_markdown.unwrap(), expected_markdown);
            assert_eq!(output_html.unwrap(), expected_html);
        }

        #[test]
        fn test_markdown_simple_ascii_only_zero_frequency() {
            let input = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr.";

            let expected_html = String::from(input);
            let expected_markdown = String::from(input);

            let output_markdown = input.process_text(0, 50., OutputType::Markdown);
            let output_html = input.process_text(0, 50., OutputType::HTML);

            assert_eq!(output_markdown.unwrap(), expected_markdown);
            assert_eq!(output_html.unwrap(), expected_html);
        }

        #[test]
        fn test_markdown_simple_ascii_only_50pct() {
            let input = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr.";

            let output_markdown = input.process_text(1, 0.5, OutputType::Markdown);
            let output_html = input.process_text(1, 0.5, OutputType::HTML);

            let expected_markdown = String::from("**Lor**em **ips**um **dol**or **si**t **am**et, **conse**tetur **sadip**scing **eli**tr.");
            let expected_html = String::from("<b>Lor</b>em <b>ips</b>um <b>dol</b>or <b>si</b>t <b>am</b>et, <b>conse</b>tetur <b>sadip</b>scing <b>eli</b>tr.");

            assert_eq!(output_html.unwrap(), expected_html);
            assert_eq!(output_markdown.unwrap(), expected_markdown);
        }

        #[test]
        fn test_markdown_simple_ascii_only_50pct_frequency_two() {
            let input = "Lorem ipsum dolor.";

            let output_markdown = input.process_text(2, 0.5, OutputType::Markdown);
            let output_html = input.process_text(2, 0.5, OutputType::HTML);

            let expected_markdown = String::from("**Lor**em ipsum **dol**or.");
            let expected_html = String::from("<b>Lor</b>em ipsum <b>dol</b>or.");

            assert_eq!(output_html.unwrap(), expected_html);
            assert_eq!(output_markdown.unwrap(), expected_markdown);
        }

        #[test]
        fn test_markdown_simple_ascii_only_50pct_frequency_three() {
            let input = "Lorem ipsum dolor sit.";

            let output_markdown = input.process_text(3, 0.5, OutputType::Markdown);
            let output_html = input.process_text(3, 0.5, OutputType::HTML);

            let expected_markdown = String::from("**Lor**em ipsum dolor **si**t.");
            let expected_html = String::from("<b>Lor</b>em ipsum dolor <b>si</b>t.");


            assert_eq!(output_html.unwrap(), expected_html);
            assert_eq!(output_markdown.unwrap(), expected_markdown);
        }


        #[test]
        fn test_markdown_simple_ascii_only_100pct() {
            let input = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr.";

            let output_markdown = input.process_text(1, 1., OutputType::Markdown);
            let output_html = input.process_text(1, 1., OutputType::HTML);

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

            let output_markdown = input.process_text(1, 0., OutputType::Markdown);
            let output_html = input.process_text(1, 0., OutputType::HTML);

            assert_eq!(output_markdown.unwrap(), expected_markdown);
            assert_eq!(output_html.unwrap(), expected_html);
        }

        #[test]
        fn test_markdown_simple_panagram_german_50pct() {
            let input = "Büß du ja zwölf Qirsch, Kämpe!";

            let output_markdown = input.process_text(1, 0.5, OutputType::Markdown);
            let output_html = input.process_text(1, 0.5, OutputType::HTML);

            let expected_markdown = String::from("**Bü**ß **d**u **j**a **zwö**lf **Qir**sch, **Käm**pe!");
            let expected_html = String::from("<b>Bü</b>ß <b>d</b>u <b>j</b>a <b>zwö</b>lf <b>Qir</b>sch, <b>Käm</b>pe!");

            assert_eq!(output_html.unwrap(), expected_html);
            assert_eq!(output_markdown.unwrap(), expected_markdown);
        }


        #[test]
        fn test_markdown_simple_panagram_german_100pct() {
            let input = "Büß du ja zwölf Qirsch, Kämpe!";

            let output_markdown = input.process_text(1, 1., OutputType::Markdown);
            let output_html = input.process_text(1, 1., OutputType::HTML);

            let expected_markdown = String::from("**Büß** **du** **ja** **zwölf** **Qirsch**, **Kämpe**!");
            let expected_html = String::from("<b>Büß</b> <b>du</b> <b>ja</b> <b>zwölf</b> <b>Qirsch</b>, <b>Kämpe</b>!");

            assert_eq!(output_markdown.unwrap(), expected_markdown);
            assert_eq!(output_html.unwrap(), expected_html);
        }
    }
}
